// use super::reg;
use super::log::{log, Levels};
use crate::Result;

pub fn print_version() {
    println!("Windows Fixer v{}", env!("CARGO_PKG_VERSION"));
}

fn print_help() {
    print_version();
    println!(
        "
Usage: windows_fixer [OPTIONS]
Options:
    -h, --help      Print this help message
    -v, --version   Print version information
"
    );
}

pub fn init(args: &Vec<String>) -> Result<()> {
    match args[1].as_str() {
        "-h" | "--help" => print_help(),
        "-v" | "--version" => print_version(),
        _ => log(
            Levels::Error,
            "Invalid option. Run with -h or --help for help.",
        ),
    }
    Ok(())
}
