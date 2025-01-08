use std::{path::Path, sync::Arc};

use include_dir::{include_dir, Dir};

static MIGRATIONS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/migrations");

use serde::Serialize;
use tokio::sync::Mutex;
use tracing::log::{debug, info};

use crate::deadpool_postgres::Pool;
use crate::tokio_postgres::error::SqlState;
use crate::Result;
use samling_clorinde::queries::misc::{migrate_revision, set_migrate_revision};

#[derive(Serialize)]
pub(crate) struct MigrationSummary {
    latest_revision: i32,
    db_revision: Option<i32>,
    up_to_date: bool,
}

impl MigrationSummary {
    pub(crate) fn up_to_date(&self) -> bool {
        self.up_to_date
    }
}

#[derive(Debug, Clone)]
pub struct Migrator {
    pool: Pool,
    latest_revision: i32,
    // TODO: Actually make use of this on errors
    cached_db_revision: Arc<Mutex<Option<i32>>>,
}

impl Migrator {
    pub(crate) fn new(pool: Pool) -> Self {
        Self {
            pool,
            latest_revision: Self::find_latest_revision(),
            cached_db_revision: Arc::new(Mutex::new(None)),
        }
    }

    pub(crate) async fn summary(&self) -> Result<MigrationSummary> {
        let db_revision = self.db_revision().await?;
        Ok(MigrationSummary {
            latest_revision: self.latest_revision,
            db_revision,
            up_to_date: db_revision.eq(&Some(self.latest_revision)),
        })
    }

    /// Migrate to the latest available database revision
    pub async fn migrate_to_latest(&self) -> Result<()> {
        info!("Migrating to latest db revision...");
        let db_revision = self.db_revision().await?;

        if let Some(r) = db_revision {
            info!("Current db revision is: #{r}...");
        } else {
            info!(
                r#"No applied database migrations detected... It is assumed that a table called "migrations" containing the non-null integer column "revision" will be created in the first migration!"#
            );
        }

        let mut client = self.pool.get().await?;
        let tx = client.transaction().await?;

        for entry in MIGRATIONS.entries() {
            let revision = Self::path_to_revision(entry.path());

            if db_revision.is_none() || revision > db_revision.unwrap() {
                info!("Applying migration #{revision}...");
                let file = entry.as_file().unwrap_or_else(|| {
                    panic!(
                        "Failed to get contents for migration file {:?}",
                        entry.path()
                    )
                });
                let query = file.contents_utf8().unwrap_or_default();
                tx.batch_execute(query).await?;
                set_migrate_revision().bind(&tx, &revision).one().await?;
                info!("Successfully applied migration #{revision}!");
            } else {
                debug!("Migration #{revision} has already been applied");
            }
        }
        tx.commit().await?;
        info!("...all done!");
        Ok(())
    }

    async fn db_revision(&self) -> Result<Option<i32>> {
        let client = self.pool.get().await?;
        match migrate_revision().bind(&client).opt().await {
            Ok(Some(db_revision)) => {
                let mut lock = self.cached_db_revision.lock().await;
                *lock = Some(db_revision);
                Ok(Some(db_revision))
            }
            Err(err) => {
                if let Some(db_error) = err.as_db_error() {
                    if db_error.code() == &SqlState::UNDEFINED_TABLE {
                        Ok(None)
                    } else {
                        Err(err.into())
                    }
                } else {
                    Err(err.into())
                }
            }
            _ => Ok(None),
        }
    }

    fn find_latest_revision() -> i32 {
        let mut max = 0;
        for entry in MIGRATIONS.entries() {
            let revision = Self::path_to_revision(entry.path());
            if revision > max {
                max = revision;
            }
        }
        max
    }

    fn path_to_revision(path: &Path) -> i32 {
        let filename = path.file_name().unwrap().to_string_lossy();
        let (raw, _) = filename
            .split_once('-')
            .unwrap_or_else(|| panic!("Failed to find migrate revision in filename {filename}"));
        raw.trim_start_matches('0')
            .parse::<i32>()
            .unwrap_or_else(|_| panic!("Failed to parse migrate revision from filename {filename}"))
    }
}
