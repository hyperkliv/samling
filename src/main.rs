#![forbid(unsafe_code)]

#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[cfg(feature = "cli")]
use std::process::{ExitCode, Termination};

#[cfg(feature = "cli")]
use samling::cli;

#[cfg(feature = "cli")]
#[repr(u8)]
enum CliResult {
    Success = 0,
    Failure = 1,
}

#[cfg(feature = "cli")]
impl Termination for CliResult {
    fn report(self) -> ExitCode {
        ExitCode::from(self as u8)
    }
}

#[cfg(not(feature = "cli"))]
fn main() {
    eprintln!("No CLI, because `cli` feature is not installed");
    std::process::exit(1);
}

#[cfg(feature = "cli")]
#[tokio::main]
async fn main() -> CliResult {
    match cli::run().await {
        Ok(()) => CliResult::Success,
        Err(err) => {
            eprintln!("{err}");
            CliResult::Failure
        }
    }
}
