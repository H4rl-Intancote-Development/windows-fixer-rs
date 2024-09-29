use std::env::args;
use windows_fixer::{cli, tui};

mod windows_fixer;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() > 1 {
        windows_fixer::cli::init(&args)?;
    } else {
        windows_fixer::tui::init()?;
    }

    Ok(())
}
