use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author = "John Crickett",
    version,
    about = "rswc, a simple wc clone in Rust"
)]
struct Arguments {
    /// Count bytes.  Count the number of bytes in the file.
    #[arg(short)]
    c: bool,

    files: Vec<String>,
}

fn main() -> std::io::Result<()> {
    let args = Arguments::parse();

    if let Some(first) = args.files.first() {
        let file = File::open(first)?;
        let reader = BufReader::new(file);
        let mut count = 0;

        for line in reader.lines() {
            count += line?.len()
        }

        let max_digits = max_digits(count);
        let s = format!("{:>width$}", count, width = max_digits as usize);

        let _ = writeln!(std::io::stdout(), "{} {}", s, first);
    }
    Ok(())
}

fn max_digits(byte_count: usize) -> usize {
    let mut max_len = 0;

    let len_bytes = byte_count.to_string().len();
    if len_bytes > max_len {
        max_len = len_bytes;
    }

    if max_len < 7 {
        max_len = 7;
    }
    return max_len + 1;
}
