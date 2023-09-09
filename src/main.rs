use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use clap::ArgAction;
use clap::{Command, Arg};
use regex::Regex;
use regex::RegexBuilder;

fn process_lines<T: BufRead + Sized>(reader: T, regex: Regex, reverse_search:bool) {
    for line in reader.lines() {
        let line = line.unwrap();
        match regex.find(&line) {
            Some(_) => { 
                if reverse_search != true {println!("{}", &line)} 
                ();
            },
            None => {
                if reverse_search != true {()}                 
                println!("{}", &line)
            },
        }
    }
}


fn main() {
    let args = Command::new("grep-lite")
        .version("0.0.1")
        .about("My version of grep-lite build in Rust. Searches for patterns.")

        .arg(Arg::new("pattern")
                .help("The pattern you are searching for.")
                .required(true))

        .arg(Arg::new("input")
                .help("File or I/O to search through.")
                .required(false))

        .arg(Arg::new("whole")
                .short('w')
                .action(ArgAction::SetTrue)
                .help("Match only whole word not substring.")
                .required(false))

        .arg(Arg::new("case sensitive")
                .short('i')
                .action(ArgAction::SetTrue)
                .help("Ignore case")
                .required(false))

        .arg(Arg::new("dont match")
                .short('v')
                .action(ArgAction::SetTrue)
                .help("Reverse search. Ignore inputted word.")
                .required(false))

        .get_matches();

    let pattern = match args.get_flag("whole") {
        true => r"\b".to_owned() + args.get_one::<String>("pattern")
                                    .expect("Pattern is missing") + r"\b",
        false => args.get_one::<String>("pattern")
                                    .expect("Pattern is missing").to_string()
    };
    let reg_ex = RegexBuilder::new(&pattern)
                        .case_insensitive(args.get_flag("case sensitive"))
                        .build()
                        .unwrap();


    let reverse_match = args.get_flag("dont match");
    if let Some(i) = args.get_one::<String>("input") {
        let open_file = File::open(i).unwrap();
        let reader = BufReader::new(open_file); 
        process_lines(reader, reg_ex, reverse_match)
    } else {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, reg_ex, reverse_match)
    }
}
