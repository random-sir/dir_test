use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use clap::{command, Parser}; //clap for cli handling

use anyhow::{Context, Result}; //anyhow for error handling

/// Expand a pattern and make directories
#[derive(Parser, Debug)]
#[command(version, about, long_about= None)]
struct Args {
    /// Pattern to be expanded.
    pattern: String,

    /// Flag which enables directory creation.
    #[arg(short = 'c')]
    non_dry_run: bool,

    /// Script to run for every directory created.
    #[arg(short = 'p', long = "post-create-hook")]
    post_create_hook: Option<PathBuf>,
}

fn main() -> Result<()> {
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

        if ch == ')' && in_parantheses {
            in_parantheses = false;
            let range: std::ops::RangeInclusive<i32> = {
                let mut range = range_str.split("..");
                let left = range.next().unwrap().parse().with_context(|| {
                    format!("Incorrectly formated range: non-numeric value at left arg")
                })?;
                let right = range.next().unwrap().parse().with_context(|| {
                    format!("Incorrectly formated range: non-numeric value at right arg")
                })?;
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

    if !non_dry_run {
        paths.iter().for_each(|a| println!("{a}"));
    }

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
            anyhow::bail!("Can't create a directory which already exists")
        }

        for path in &paths {
            fs::create_dir(path).with_context(|| format!("Error creating directory: {}", path))?;
        }
    }
    //post-create hook
    if let Some(hook_path) = args.post_create_hook {
        let hook_fullpath = fs::canonicalize(&hook_path)
            .with_context(|| format!("Couldn't find file: {:?}", &hook_path))?;
        for path in &paths {
            let program_output = Command::new(&hook_fullpath)
                .current_dir(path)
                .env("CREATED_DIR", path)
                .output()
                .with_context(|| format!("Couldn't execute file: {:?}", &hook_path))?;
            println!("{}", String::from_utf8(program_output.stdout)?);
        }
    }

    Ok(())
}
