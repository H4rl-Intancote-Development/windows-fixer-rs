use owo_colors::OwoColorize;
pub enum Levels {
    Info,
    Warning,
    Error,
}

pub fn log(level: Levels, message: &str) {
    let program_name = env!("CARGO_PKG_NAME");

    match level {
        Levels::Info => println!("{}: {}", program_name.blue(), message),
        Levels::Warning => println!("{}: {}", program_name.yellow(), message),
        Levels::Error => eprintln!("{}: {}", program_name.red(), message),
    }
}
