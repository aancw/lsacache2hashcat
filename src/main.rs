// Copyright (c) 2022 Petruknisme
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

extern crate clap;
extern crate colored;

use clap::Parser;
use colored::Colorize;
use std::{
    fs::{copy, create_dir_all, File},
    io::{self, Write},
    path::Path,
};

#[derive(Parser)]
#[clap(name = "lsacache2hashcat")]
#[clap(author = "Petruknisme <me@petruknisme.com>")]
#[clap(version = "1.0")]
#[clap(about = "Give me lsadump:cache from mimikatz, I will transform it to DCC2 Hashcat compatible. Useful for so many credentials cache", long_about = None)]

struct Cli {
    /// Lsadump cache output from mimikatz
    #[clap(short, long)]
    file: String,
}

fn main() {
    let cli = Cli::parse();
    let dll_loc = cli.dll;
    let payload_loc = cli.payload;
    let auto = cli.auto;
}