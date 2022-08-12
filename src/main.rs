use std::env;

use regex::Regex;

fn main() {
    // TODO: update the behavior when args is empty:
    //       0. We should expect user to run this after creating a new rust leetcode solution
    //       1. Edit the file name to line up with the rust module name rule
    //       2. Add the file name to lib.rs
    //       3. Add the template of struct Solution and test to the new file

    let args: Vec<String> = env::args().collect();
    let default_filename = String::from("1");

    let filename: &String = match args.get(1) {
        Some(filename) => filename,
        None => {
            println!("Using default filename: {}", default_filename);
            &default_filename
        }
    };
    let new_filename = leetcode_name_to_rust_module(filename);
    cli_clipboard::set_contents(new_filename.to_owned()).unwrap();
    println!("`{}' is copied to clipboard", new_filename);
}

#[allow(dead_code)]
fn naive_leetcode_name_to_rust_module(old_name: &str) -> String {
    // check if old name is {number}.{word}-{word}-{...}.rs
    let mut split = old_name.split(".");
    // assert first part is number
    let number = split.next().unwrap();
    let number = number.parse::<i32>().unwrap();
    // assert second part is {word}-{word}-...
    let name = split.next().unwrap();
    let mut words = name.split("-");
    while let Some(word) = words.next() {
        // check if word is formed by lowercase letters
        if word
            != word
                .chars()
                .filter(|c| c.is_lowercase())
                .collect::<String>()
        {
            panic!("{} is not a valid name to convert", old_name);
        }
    }

    match split.next() {
        Some(_ext) => {
            if _ext != "rs" {
                panic!("{} is not a valid name to convert", old_name);
            }
            _ext
        }
        None => "rs",
    };

    let new_name = format!(
        "q{}_{}.rs",
        number,
        name.split("-").collect::<Vec<&str>>().join("_")
    );
    new_name
}

fn leetcode_name_to_rust_module(leetcode_name: &str) -> String {
    // check with regex if old_name is {number}.{word}-{word}-{...}.rs
    // copilot knows how to do regex
    // let re = Regex::new(r"^(\d+)\.([a-z]+(?:-[a-z]+)*)\.rs$").unwrap();
    let test_re = Regex::new(r"^(\d+)\.([\w]+)(-[a-z]+)*(\.rs)?$").unwrap(); // my regex
    assert!(test_re.is_match(leetcode_name));

    let mut module_name: String = leetcode_name
        .chars()
        .map(|x| match x {
            '.' | '-' => '_',
            _ => x,
        })
        .collect();
    if !module_name.ends_with(".rs") {
        module_name.push_str(".rs");
    }
    format!("q{}", module_name)
}
