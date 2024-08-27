use std::{
    env, fs,
    path::Path,
    process::{self, Command},
};

use clap::{command, Parser}; //clap for cli handling

/// Expand a pattern and make directories
#[derive(Parser, Debug)]
#[command(version, about, long_about= None)]
struct Args {
    /// Pattern to be expanded
    pattern: String,

    /// Flag which enables directory creation
    #[arg(short = 'c')]
    non_dry_run: bool,

    /// Flag which executes the (currently placeholder) post-create hook
    #[arg(short = 'p')]
    post_create_hook: bool,
}

fn main() {
    // let mut args: Vec<String> = env::args().collect();
    let args = Args::parse();

    let non_dry_run = args.non_dry_run;

    let raw_path = args.pattern;

    let mut chars = raw_path.chars();
    let mut paths: Vec<String> = vec![String::with_capacity(raw_path.capacity())];

    let mut in_parantheses = false;
    let mut range_str = String::with_capacity(4);
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            //escaping char
            paths.iter_mut().for_each(|a| a.push(chars.next().unwrap())); //Panics if escaping at end of string
            continue;
        }
        if ch == '(' {
            in_parantheses = true;
            continue;
        }

        if ch == ')' {
            in_parantheses = false;
            let range: std::ops::RangeInclusive<i32> = {
                let mut range = range_str.split("..");
                let left = range.next().unwrap().parse().unwrap();
                let right = range.next().unwrap().parse().unwrap();
                left..=right
            };

            let count = paths.len();
            for i in range {
                for j in 0..count {
                    let base = paths[j].clone();
                    paths.push(base + &i.to_string());
                }
            }
            paths.drain(0..count);
            range_str.clear();

            continue;
        }

        if in_parantheses {
            range_str.push(ch);
            continue;
        }

        paths.iter_mut().for_each(|a| a.push(ch));
    }

    paths.iter().for_each(|a| println!("{a}"));

    if non_dry_run {
        //Error if any Directory already exists
        let mut any_dir_exists = false;
        for path in &paths {
            if Path::new(&path).exists() {
                eprintln!("Directory: {path} already exists");
                any_dir_exists = true;
            }
        }

        if any_dir_exists {
            process::exit(1);
        }

        for path in &paths {
            fs::create_dir(path).unwrap();
        }
    }
    //post-create hook
    if args.post_create_hook {
        for path in &paths {
            let echo_test = Command::new("./call_hook.sh").arg(path).output().unwrap();
            println!("{}", String::from_utf8(echo_test.stdout).unwrap());
        }
    }
}
