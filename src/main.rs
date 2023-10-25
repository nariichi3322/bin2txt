mod bin2txt;
mod config;
mod debug;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    debug_dbg!(&args);

    let config = config::Config::build(&args).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });
    debug_dbg!(&config);

    let result_size = bin2txt::convert(config).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    println!("Finished. Size: {result_size}");
}
