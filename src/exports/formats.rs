use std::collections::VecDeque;

use indexmap::{IndexMap, IndexSet};
use schemars::JsonSchema;
use serde::Deserialize;
use smol_str::SmolStr;

use crate::Result;

use super::{
    export::{Column, ExportRow},
    Value,
};

static MAX_XLSX_COLUMN_WIDTH: usize = 50;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ExportFormat {
    Json,
    Csv,
    Xlsx,
}

impl ExportFormat {
    pub(super) fn file_extension(&self) -> &'static str {
        use ExportFormat::*;
        match self {
            Json => ".json",
            Xlsx => ".xlsx",
            Csv => ".csv",
        }
    }

    pub(super) fn export(
        &self,
        columns: &IndexSet<Column>,
        mut rows: VecDeque<ExportRow>,
        cell_separator: &str,
    ) -> Result<bytes::Bytes> {
        match self {
            Self::Json => {
                let mut output = Vec::with_capacity(rows.len());
                output.push(b'[');
                for (i, (row, columns)) in
                    rows.into_iter().zip(std::iter::repeat(columns)).enumerate()
                {
                    if i > 0 {
                        output.push(b',');
                    }
                    let out = IndexMap::<SmolStr, Value>::from_iter(
                        columns.iter().map(|c| c.name()).zip(row.inner()),
                    );
                    serde_json::to_writer_pretty(&mut output, &out)?;
                }
                output.push(b']');
                Ok(bytes::Bytes::from(output))
            }
            Self::Csv => {
                let output = Vec::with_capacity(rows.len() + 1);
                let mut writer = csv::WriterBuilder::new().from_writer(output);
                let mut header_record = csv::ByteRecord::new();
                for column in columns {
                    header_record.push_field(column.name().as_bytes());
                }
                writer.write_record(&header_record)?;
                while let Some(row) = rows.pop_front() {
                    let record: csv::ByteRecord = row.finalized_strings(cell_separator).into();
                    writer.write_record(&record)?;
                }
                writer.flush()?;
                Ok(writer.into_inner().unwrap().into())
            }
            Self::Xlsx => {
                let mut workbook = rust_xlsxwriter::Workbook::new();
                let colfmt = rust_xlsxwriter::Format::new().set_bold();
                let cellfmt = rust_xlsxwriter::Format::new()
                    .set_align(rust_xlsxwriter::XlsxAlign::Top)
                    .set_text_wrap();

                // Add a worksheet to the workbook.
                let worksheet = workbook.add_worksheet();

                let mut max_column_widths = Vec::from_iter(columns.iter().map(|c| c.name().len()));

                for (col_index, col) in columns.iter().enumerate() {
                    worksheet.write_string(0, col_index as u16, &col.name(), &colfmt)?;
                }

                for (row_index, row) in rows.into_iter().enumerate() {
                    let row_index = (row_index + 1) as u32;
                    let mut max_line_count = 1;
                    for (col_index, _) in columns.iter().enumerate() {
                        let value = row.get(col_index).unwrap();
                        let string_value = value.finalize(cell_separator);
                        let line_count = string_value.lines().count() + 1;
                        worksheet.write_string(
                            row_index,
                            col_index as u16,
                            &string_value,
                            &cellfmt,
                        )?;
                        if line_count > max_line_count {
                            max_line_count = line_count;
                        }
                        let column_width = string_value
                            .lines()
                            .map(|line| line.len())
                            .max()
                            .unwrap_or_default();
                        if column_width > max_column_widths[col_index] {
                            if column_width > MAX_XLSX_COLUMN_WIDTH {
                                max_column_widths[col_index] = MAX_XLSX_COLUMN_WIDTH;
                            } else {
                                max_column_widths[col_index] = column_width;
                            }
                        }
                    }
                    let extra = if max_line_count > 5 {
                        max_line_count = 5;
                        8
                    } else {
                        5
                    };
                    worksheet.set_row_height(row_index, (max_line_count * 11 + extra) as f64)?;
                }

                for (col_index, width) in max_column_widths.into_iter().enumerate() {
                    worksheet.set_column_width(col_index as u16, (width + 1) as f64)?;
                }

                worksheet.set_freeze_panes(1, 0)?;

                let buf = workbook.save_to_buffer()?;
                Ok(buf.into())
            }
        }
    }
}
