#![warn(missing_docs)]
#![deny(unsafe_code)]
//! `TodayTmp`
//!
//! Application for automating create a temporary directory according current day with creating nice
//! symlink. Result of the program is a path to temporary directory which can be evaluated by
//! shell.
//!
//! To see usage, please read Readme.md file in source root. There are also installation
//! instructions.
//!
//!

extern crate chrono;

use chrono::prelude::*;
use std::io;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

struct Options {
    pub dir_path: PathBuf,
    pub today_link_path: PathBuf,
}

fn main() {
    let options = prepare();
    if !path_exists(&options.dir_path) {
        match new_day(&options) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Error occurred during IO operations.");
                std::process::exit(1);
            }
        }
    }
    println!("{}", &options.today_link_path.as_path().display());
}

fn prepare() -> Options {
    let mut home_dir = get_home_dir();
    home_dir.push("tmp");
    let mut link_path = home_dir.clone();
    link_path.push("today");
    let mut options = Options {
        dir_path: home_dir,
        today_link_path: link_path,
    };
    let now = Local::now();
    let today_path = format!(
        "{}/{}/{}/",
        now.year(),
        add_zero(&now.month().to_string()),
        add_zero(&now.day().to_string())
    );
    options.dir_path.push(today_path);
    options
}

fn path_exists(path_buf: &PathBuf) -> bool {
    Path::new(&path_buf).exists()
}

fn add_zero(value: &str) -> String {
    let mut result = String::with_capacity(2);

    if value.len() == 1 {
        result.push('0')
    }
    result.push_str(value);
    result
}

fn get_home_dir() -> PathBuf {
    match std::env::home_dir() {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(""),
    }
}

fn create_dir(opt: &Options) -> io::Result<()> {
    let dir = &opt.dir_path;
    let dir = dir.as_path();
    fs::create_dir_all(dir)
}

fn create_link(opt: &Options) -> io::Result<()> {
    let link_path = &opt.today_link_path.as_path();
    if path_exists(&opt.today_link_path) {
        fs::remove_file(link_path)?;
    }

    symlink(&opt.dir_path, link_path)
}

fn new_day(opt: &Options) -> io::Result<()> {
    create_dir(opt)?;
    create_link(opt)
}
