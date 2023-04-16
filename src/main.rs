use clap::Parser;
use itertools::Itertools;
use regex;
use std::fs::{read_to_string, File};
use std::io::{stdin, BufRead, BufReader};

use std::path::PathBuf;
enum Group {
    Number(usize),
    Name(String),
}
#[derive(Parser)]
#[clap(author = "Bryan Hu", version = "0.1.0", about = "Regex-based text extraction utility", long_about = None)]
struct Cli {
    /// The regex to use
    #[clap(value_parser)]
    regex: String,
    /// The file to execute on [default: stdin]
    #[clap(value_parser, value_name = "FILE")]
    file: Option<PathBuf>,
    /// The capture group number to print
    #[clap(short, long, value_parser, default_value_t = 0)]
    group: usize,
    /// The name of the capture group to print. Has precedence over --group
    #[clap(short = 'n', long, value_parser, value_name = "NAME")]
    group_name: Option<String>,
    /// Execute on the whole thing instead of line-by-line [default: false]
    #[clap(short, long, value_parser, default_value_t = false)]
    doc: bool,
}

fn get_file_or_stdin_contents(file: Option<PathBuf>) -> Result<String, &'static str> {
    match file {
        Some(path) => read_to_string(path).or(Err("file does not exist")),
        None => Ok(stdin().lines().map(|x| x.expect("stdin bruh")).join("\n")),
    }
}
fn get_file_or_stdin_reader(file: Option<PathBuf>) -> Result<Box<dyn BufRead>, &'static str> {
    match file {
        Some(path) => File::open(path).map_or(Err("file does not exist"), |f| {
            Ok(Box::new(BufReader::new(f)))
        }),
        None => Ok(Box::new(BufReader::new(stdin()))),
    }
}
fn get_cap_groups(re: &regex::Regex, group: &Group, contents: String) -> Option<String> {
    let capture = re.captures(contents.as_str())?;
    match group {
        Group::Number(group_number) => Some(String::from(capture.get(*group_number)?.as_str())),
        Group::Name(group_name) => {
            Some(String::from(capture.name((*group_name).as_str())?.as_str()))
        }
    }
}

fn _main() -> i32 {
    // TODO: A "ok or error with message" macro
    let cli = Cli::parse();
    let re = regex::Regex::new(cli.regex.as_str());
    let group = if let Some(name) = cli.group_name {
        Group::Name(name)
    } else {
        Group::Number(cli.group)
    };
    match re {
        Err(err) => {
            match err {
                regex::Error::Syntax(message) => eprintln!("{}", message),
                _ => unreachable!(),
            }
            return 1;
        }

        Ok(matcher) => {
            if cli.doc {
                // read all
                match get_file_or_stdin_contents(cli.file) {
                    Ok(contents) => {
                        let capture = get_cap_groups(&matcher, &group, contents);
                        if let Some(val) = capture {
                            println!("{}", val.as_str());
                        } else {
                            return 1;
                        }
                    }
                    Err(message) => {
                        eprintln!("Error: {}", message);
                        return 1;
                    }
                }
            } else {
                // line-by-line
                match get_file_or_stdin_reader(cli.file) {
                    Ok(reader) => {
                        for line in reader.lines() {
                            let capture = get_cap_groups(&matcher, &group, line.unwrap());
                            if let Some(val) = capture {
                                println!("{}", val.as_str());
                            } else {
                                continue;
                            }
                        }
                    }
                    Err(message) => {
                        eprintln!("Error: {}", message);
                        return 1;
                    }
                }
            }
        }
    }
    return 0;
}
fn main() {
    std::process::exit(_main());
}
