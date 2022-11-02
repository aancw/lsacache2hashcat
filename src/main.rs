// Copyright (c) 2022 Petruknisme
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

extern crate clap;
extern crate colored;

use clap::Parser;
use colored::Colorize;
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Parser)]
#[command(name = "lsacache2hashcat")]
#[command(author = "Petruknisme <me@petruknisme.com>")]
#[command(version = "1.0")]
#[command(about = "Give me lsadump::cache from mimikatz, I will transform it to DCC2 Hashcat compatible. Useful for dumping so many credentials caches", long_about = None)]
struct Cli {
    /// Lsadump cache output from mimikatz
    #[clap(short, long)]
    file: String,
}

struct User {
    username: String,
}

struct DCC2 {
    hash: String,
}

fn main() {
    let cli = Cli::parse();
    let file = cli.file;
    let re_iter = Regex::new(
        r"\(([^)]+)\)"
    ).unwrap();
    let re_user = Regex::new(
        r"([^\\]+$)"
    ).unwrap();
    let re_hash = Regex::new(
        r"([^ :]+$)"
    ).unwrap();
    let mut iteration: &str;
    let mut user: Vec<User> = Vec::new();
    let mut dcc2_hash: Vec<DCC2> = Vec::new();

    if let Ok(lines) = read_lines(file) {

        for line in lines {
            if let Ok(i) = line {
                if i.contains("Iteration") {
                    let text_re = re_iter.captures(&i).unwrap();
                    iteration = text_re.get(1).map_or("", |m| m.as_str());                
                }
                if i.contains("User"){
                    let text_re = re_user.captures(&i).unwrap();
                    let result_user = text_re.get(1).map_or("", |m| m.as_str());
                    let user_struct = User { username: result_user.to_string() };
                    let _= user.push(user_struct);
                }
                if i.contains("MsCacheV2"){
                    let text_re = re_hash.captures(&i).unwrap();
                    let result_dcc2_hash = text_re.get(1).map_or("", |m| m.as_str());
                    let hash_struct = DCC2 { hash: result_dcc2_hash.to_string()};
                    let _= &dcc2_hash.push(hash_struct);
                }

            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}