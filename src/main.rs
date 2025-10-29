use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead, BufReader};

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

    /// Count words.  Count the number of words in the file.
    #[arg(short)]
    w: bool,

    /// Count lines.  Count the number of lines in the file.
    #[arg(short)]
    l: bool,

    /// Count characters.  Count the number of characters in the file.
    #[arg(short)]
    m: bool,

    files: Vec<String>,
}

fn main() -> std::io::Result<()> {
    let args = Arguments::parse();

    let none = !(args.l || args.w || args.c || args.m);

    let show_l = args.l || none;
    let show_w = args.w || none;
    let show_c = args.c || none;
    let show_m = args.m;

    if args.files.is_empty() {
        let mut stdin_lock = io::stdin().lock();
        let _ = counts(&mut stdin_lock, "", show_l, show_w, show_c, show_m);
    } else {
        for path in &args.files {
            let mut reader = BufReader::new(File::open(path)?);
            let _ = counts(&mut reader, path, show_l, show_w, show_c, show_m);
        }
    }
    Ok(())
}

fn counts<R: BufRead>(
    reader: &mut R,
    filename: &str,
    show_l: bool,
    show_w: bool,
    show_c: bool,
    show_m: bool,
) -> io::Result<()> {
    let mut byte_count = 0;
    let mut line_count = 0;
    let mut word_count = 0;
    let mut char_count = 0;

    let mut buf = String::new();
    loop {
        buf.clear();
        let n = reader.read_line(&mut buf)?; // n is bytes read; includes '\n' if present
        if n == 0 {
            break;
        }

        byte_count += n; // bytes, including newline
        line_count += 1;
        word_count += buf.split_whitespace().count();
        char_count += buf.chars().count(); // includes newline as one character
    }

    let max_digits = max_digits(byte_count, char_count, line_count, word_count);
    let byte_count_str = format!("{:>width$}", byte_count, width = max_digits);
    let line_count_str = format!("{:>width$}", line_count, width = max_digits);
    let word_count_str = format!("{:>width$}", word_count, width = max_digits);
    let char_count_str = format!("{:>width$}", char_count, width = max_digits);

    if show_l {
        let _ = write!(io::stdout(), "{} ", line_count_str);
    }
    if show_w {
        let _ = write!(io::stdout(), "{} ", word_count_str);
    }
    if show_c {
        let _ = write!(io::stdout(), "{} ", byte_count_str);
    }
    if show_m {
        let _ = write!(io::stdout(), "{} ", char_count_str);
    }

    let _ = writeln!(io::stdout(), "{}", filename);

    Ok(())
}

fn max_digits(byte_count: usize, char_count: usize, line_count: usize, word_count: usize) -> usize {
    let mut max_len = 0;

    let len_bytes = byte_count.to_string().len();
    if len_bytes > max_len {
        max_len = len_bytes;
    }

    let len_chars = char_count.to_string().len();
    if len_chars > max_len {
        max_len = len_chars;
    }

    let len_lines = line_count.to_string().len();
    if len_lines > max_len {
        max_len = len_lines;
    }

    let len_words = word_count.to_string().len();
    if len_words > max_len {
        max_len = len_words;
    }

    if max_len < 7 {
        max_len = 7;
    }
    max_len + 1
}
