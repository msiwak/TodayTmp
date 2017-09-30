#![warn(missing_docs)]
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
    if path_exists(&options.dir_path) {
        // Todo: go to directory
    } else {
        create_dir(&options);
        create_link(&options);
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
    let today_path = format!("{}/{}/{}/",
                             now.year(),
                             add_zero(now.month().to_string()),
                             add_zero(now.day().to_string()));
    options.dir_path.push(today_path);
    options
}

fn path_exists(path_buf: &PathBuf) -> bool {
    Path::new(&path_buf).exists()
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
        None => PathBuf::from(""),
    };
    dir
}

fn create_dir(opt: &Options) {
    let dir = &opt.dir_path;
    let dir = dir.as_path();
    let result: io::Result<()> = fs::create_dir_all(dir);
    match result {
        Ok(_) => {}
        Err(_) => {}
    }
}

fn create_link(opt: &Options) {
    let link_path = &opt.today_link_path.as_path();
    let mut result: io::Result<()>;
    if path_exists(&opt.today_link_path) {
        result = fs::remove_file(link_path);
        match result {
            Ok(_) => {}
            Err(_) => {}
        }
    }

    result = symlink(&opt.dir_path, link_path);
    match result {
        Ok(_) => {}
        Err(_) => {}
    }
}
