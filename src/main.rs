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
    let mut iter_text: Vec<String> = Vec::new();
    let mut user: Vec<String> = Vec::new();
    let mut dcc2_hash: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(file) {

        for line in lines {
            if let Ok(i) = line {
                if i.contains("Iteration") {
                    let text_re = re_iter.captures(&i).unwrap();
                    let iter_result = text_re.get(1).map_or("", |m| m.as_str());
                    iter_text.push(iter_result.to_string());
                }
                if i.contains("User"){
                    let text_re = re_user.captures(&i).unwrap();
                    let result_user = text_re.get(1).map_or("", |m| m.as_str());
                    user.push(result_user.to_string());
                }
                if i.contains("MsCacheV2"){
                    let text_re = re_hash.captures(&i).unwrap();
                    let result_dcc2_hash = text_re.get(1).map_or("", |m| m.as_str());
                    dcc2_hash.push(result_dcc2_hash.to_string());
                }
            }
        }

        user.iter().enumerate().for_each(|(index, value)| {
            println!("$DCC2${}#{}#{}", iter_text[0], value, dcc2_hash[index]);
        });
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}