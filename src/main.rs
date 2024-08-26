use std::{env, fs, ops::RangeFrom, path::Path, process::Command};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let raw_path = &mut args[1];

    // let mut chars = raw_path.chars();
    // // let a = chars.position(|c| c == '(');
    // let b = chars.position(|c| c == ')');

    let a = raw_path.find('(').unwrap();
    let b = raw_path.find(')').unwrap();

    let range: String = raw_path.drain(a..=b).collect();

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{}", raw_path);
    println!("{}", range);

    let mut p = range.match_indices(|c: char| c.is_ascii_digit());

    let test = p.next();

    println!("{:?}", p);
    println!("{:?}", test);

    // //Create cycle
    // for i in 1..10 {
    //     let path = "/tmp/test_".to_owned() + &i.to_string();

    //     if Path::new(&path).exists() {
    //         println!("Deleted: {}", &path);
    //         fs::remove_dir(path).unwrap();
    //     } else {
    //         println!("Created: {}", &path);
    //         fs::create_dir(path).unwrap();
    //     }
    // }

    // //post-create hook
    // for i in 1..10 {
    //     let path = "/tmp/test_".to_owned() + &i.to_string();
    //     let echo_test = Command::new("./test.sh").arg(path).output().unwrap();
    //     println!("{}", String::from_utf8(echo_test.stdout).unwrap());
    // }
}
