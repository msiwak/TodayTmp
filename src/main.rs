extern crate chrono;

use chrono::prelude::*;
use std::io;
use std::fs;
use std::path::{Path, PathBuf};

struct Options {
    pub tmp_dir: String,
    pub dir_path: PathBuf
}

fn main() {
    let options = prepare();
    if dir_exists(&options) {
        // Todo: go to directory
    } else {
        create_dir(&options);
    }
}

fn prepare() -> Options {
    let mut home_dir = get_home_dir();
    home_dir.push("tmp");
    let mut options = Options {
        tmp_dir: String::new(),
        dir_path: home_dir,
    };
    let now = Local::now();
    let today_path = format!("{}/{}/{}/", now.year(), add_zero(now.month().to_string()),
                             add_zero(now.day().to_string()));
    options.dir_path.push(today_path);
    options
}

fn dir_exists(opt: &Options) -> bool {
    Path::new(&opt.dir_path).exists()
}

fn add_zero(value: String) -> String {
    let mut result = String::with_capacity(2);

    if value.len() == 1 {
        result.push('0')
    }
    result.push_str(value.as_str());
    result
}

fn get_home_dir() -> PathBuf {
    let dir: PathBuf = match std::env::home_dir() {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from("")
    };
    dir
}

fn create_dir(opt: &Options) {
    let dir = &opt.dir_path;
    let dir = dir.as_path();
    let result: io::Result<()> = fs::create_dir_all(dir);
    match result {
        Ok(_) => {},
        Err(_) => { }
    }
}
