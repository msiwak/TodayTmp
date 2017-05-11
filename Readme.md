[![Build Status](https://travis-ci.org/msiwak/TodayTmp.svg?branch=master)](https://travis-ci.org/msiwak/TodayTmp)

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
* create or update ~/tmp/today link which is pointing to today's temporary directory (but without any error message and/or handling)
* change your current working directory to ~/tmp/today on every `ttmp` run

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

If you want to use `ttmp` to automatically enter to today's directory, copy and paste the content of `files/addToRc.sh` to your rc files.
For bash it will be `~/.bashrc`, for zsh `~/.zshrc`. For both shells it was tested. 

```bash
cat files/addToRc.sh >> YOUR_RC_FILE
```


## Usage

When you type `ttmp`, program will create a tree of directories. For example, for the date: 25th of May 2017 it will create 
`~/tmp/2017/05/25`.

If you updated your `.bash_rc` or `.zshrc` file, application will also change your current working directory to the freshly created one.

You can use `ttmp` multiple times, it will not create more than one directory, but it will change your directory.

Example:

```bash
~$ ls tmp 
ls: tmp: No such file or directory
~$ ttmp
~/tmp/today$ pwd
~/tmp/today
~/tmp/today$ tree ~/tmp
~/tmp
├── 2017
|   |
|   └── 05
|       |
|       └── 11
└── today -> ~/tmp/2017/05/11/ 
~/tmp/today$ cd /var/log
/var/log$ ttmp
~/tmp/today$ tree ~/tmp
~/tmp
├── 2017
|   |
|   └── 05
|       |
|       └── 11
└── today -> ~/tmp/2017/05/11/
```
