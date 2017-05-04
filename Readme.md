# TodayTmp

## Description

This tool is created because of my laziness ;)

Almost everyday I'm creating some kind of tmp directory to test something. Usually, I was creating directories in different formats, like:
```bash
~/tmp/20170525
~/tmp/1, ~/tmp2
~/tmp/2017/05/25
```
and so on. First, it is torturous for me to create such directories every day. What is more, cd to such folders was not such easy thing.
So, I decided to create `~/tmp/today` link which was always point to the current day directory.

TodayTmp project is being created to automate all of the processes.
 
So far, tool can:
* create ~/tmp/YEAR/MONTH/DAY directory (but without any error message and/or handling)

What is planned to be done:
* automation of changing the target of `~/tmp/today` link
* typing `ttmp` should not only create directory if such does not exists but also go to it. No more `cd ~/tmp/today` necessary


## Installation

There are two ways to install TodayTmp: downloading binary file or cloning repo and compiling 
it from sources.

### Downloading binary file

###### TODO: download instructions

### Compiling

You need to have Rust installed on your machine. The easiest way is to go to `https://rustup.rs/` 
and follow the instructions.
 
Clone the repo, and type:

```bash
cargo build --release
```

and then copy binary to one of the folders you have in your PATH, ex. `/usr/local/bin`.

```bash
cp target/release/ttmp /usr/local/bin
```

## Usage

When you type `ttmp`, program will create a tree of directories. For example, for the date: 25th of May 2017 it will create 
`~/tmp/2017/05/25`.
