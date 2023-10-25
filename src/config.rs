use std::ffi::OsStr;
use std::path::Path;

#[derive(Debug)]
pub struct Config {
    pub input_file: String,
    pub output_file: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Please give input file path.");
        }

        let input_file = args[1].clone();
        let input_file_path = Path::new(&input_file);
        let output_file = input_file_path
            .with_extension("")
            .as_os_str()
            .to_os_string()
            .into_string()
            .unwrap()
            + if input_file_path.extension().and_then(OsStr::to_str).unwrap() == "txt" {
                "_out.txt"
            } else {
                ".txt"
            };

        Ok(Config {
            input_file,
            output_file,
        })
    }
}
