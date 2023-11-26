use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{Arg, Command};
use regex::Regex;

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();

        if let Some(_) = re.find(&line) {
            println!("{}", line);
        }
    }
}

fn main() {
    let args = Command::new("grep-lite")
        .version("0.1.0")
        .author("jaemin kim")
        .about("lite grep example")
        .arg(
            Arg::new("pattern")
                .short('p')
                .long("pattern")
                .required(true)
                .help("pattern to find"),
        )
        .arg(Arg::new("input").help("file to search").required(false))
        .get_matches();

    let pattern = args.get_one::<String>("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();

    let no_input = String::from("_");
    let input = args.get_one::<String>("input").unwrap_or(&no_input);

    match &input[..] {
        "_" => {
            let stdin = io::stdin();
            let reader = stdin.lock();
            process_lines(reader, re);
        }
        _ => {
            let f = File::open(input).unwrap();
            let reader = BufReader::new(f);
            process_lines(reader, re);
        }
    }
}
