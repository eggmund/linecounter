mod counter;
mod options;

use std::io;
use std::process;

use structopt::StructOpt;

use crate::counter::LineCounter;
use crate::options::Options;

fn run() -> io::Result<()> {
    let ops = Options::from_args();

    let mut line_counter = LineCounter::new(ops);
    
    line_counter.get_line_count()?;
    println!("{}", line_counter);

    Ok(())
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("linecount: error: {}", e);
            if let Some(os_error) = e.raw_os_error() {
                process::exit(os_error)
            } else {
                process::exit(1)
            }
        }
    }
}