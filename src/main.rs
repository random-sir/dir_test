use std::{env, fs, path::Path, process::Command};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let raw_path = &mut args[1];

    // let mut chars = raw_path.chars();
    // // let a = chars.position(|c| c == '(');
    // let b = chars.position(|c| c == ')');

    let a = raw_path.find('(').unwrap();
    let b = raw_path.find(')').unwrap();

    let path: String = raw_path.drain(a..=b).collect();

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{}", path)

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
