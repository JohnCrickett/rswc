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

    #[arg(short)]
    l: bool,

    files: Vec<String>,
}

fn main() -> std::io::Result<()> {
    let args = Arguments::parse();

    if let Some(first) = args.files.first() {
        let file = File::open(first)?;
        let reader = BufReader::new(file);
        let mut byte_count = 0;
        let mut line_count = 0;

        for line in reader.lines() {
            byte_count += line?.len();
            line_count += 1;
        }

        let max_digits = max_digits(byte_count, line_count);
        let byte_count_str = format!("{:>width$}", byte_count, width = max_digits as usize);
        let line_count_str = format!("{:>width$}", line_count, width = max_digits as usize);

        if args.l {
            let _ = write!(std::io::stdout(), "{} ", line_count_str);
        }
        if args.c {
            let _ = write!(std::io::stdout(), "{} ", byte_count_str);
        }

        let _ = writeln!(std::io::stdout(), "{}", first);
    }
    Ok(())
}

fn max_digits(byte_count: usize, line_count: usize) -> usize {
    let mut max_len = 0;

    let len_bytes = byte_count.to_string().len();
    if len_bytes > max_len {
        max_len = len_bytes;
    }

    let len_lines = line_count.to_string().len();
    if len_lines > max_len {
        max_len = len_lines;
    }

    if max_len < 7 {
        max_len = 7;
    }
    max_len + 1
}
