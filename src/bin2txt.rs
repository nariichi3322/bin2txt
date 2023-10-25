use crate::config;
use crate::debug_dbg;
use std::error::Error;
use std::fs;
use std::io::{BufWriter, Write};

pub fn convert(config: config::Config) -> Result<usize, Box<dyn Error>> {
    let bytes = fs::read(config.input_file)?;
    let output_file = fs::File::create(config.output_file).unwrap();
    let mut output_file = BufWriter::new(output_file);

    debug_dbg!(&output_file);

    for (idx, byte) in bytes.iter().enumerate() {
        write!(
            &mut output_file,
            "{:#04X},{}",
            byte,
            if (idx + 1) % 16 == 0 { "\r\n" } else { " " }
        )
        .unwrap();

        crate::debug_print!(
            "{:#04X},{}",
            byte,
            if (idx + 1) % 16 == 0 { "\r\n" } else { " " }
        );
    }

    Ok(bytes.len())
}
