use std::{env, process};
mod file_handlers;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = filer::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    file_handlers::handle_cmd(&config.cmd, &config.filename).unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        process::exit(1);
    });
}
