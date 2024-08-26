use std::{char, env, fs, ops::RangeFrom, path::Path, process::Command};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let raw_path = &mut args[1];

    let mut chars = raw_path.chars();
    let mut path: Vec<String> = vec![String::with_capacity(raw_path.capacity())];

    let mut in_parantheses = false;
    let mut range_str = String::with_capacity(4);
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            //escaping char
            path.iter_mut().for_each(|a| a.push(chars.next().unwrap())); //Panics if escaping at end of string
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

            let count = path.len();
            for i in range {
                for j in 0..count {
                    let base = path[j].clone();
                    path.push(base + &i.to_string());
                }
            }
            path.drain(0..count);
            range_str.clear();

            continue;
        }

        if in_parantheses {
            range_str.push(ch);
            continue;
        }

        path.iter_mut().for_each(|a| a.push(ch));
    }

    println!("{}", range_str);

    path.iter().for_each(|a| println!("{a}"));

    // //post-create hook
    // for i in 1..10 {
    //     let path = "/tmp/test_".to_owned() + &i.to_string();
    //     let echo_test = Command::new("./test.sh").arg(path).output().unwrap();
    //     println!("{}", String::from_utf8(echo_test.stdout).unwrap());
    // }
}
