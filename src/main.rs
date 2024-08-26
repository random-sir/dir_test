use std::{
    char,
    env::{self, args},
    fs,
    ops::RangeFrom,
    path::Path,
    process::{self, Command},
};

fn main() {
    let mut args: Vec<String> = env::args().collect();

    let mut non_dry_run = false;
    if args.contains(&"-c".to_string()) {
        non_dry_run = true;
    }
    let raw_path = &mut args[1];

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

        for path in paths {
            fs::create_dir(path).unwrap();
        }
    }
    // //post-create hook
    // for i in 1..10 {
    //     let path = "/tmp/test_".to_owned() + &i.to_string();
    //     let echo_test = Command::new("./test.sh").arg(path).output().unwrap();
    //     println!("{}", String::from_utf8(echo_test.stdout).unwrap());
    // }
}
