use std::env;
use std::io::Write;
use std::path::Path;

use regex::Regex;
/// The main entry point of the program.
/// ## Arguments
/// * `path` (optional) - The path of the root of the rust repository.
///
/// ## Functionality
/// Rename all file with leetcode default name to rust module name.
/// Add the module name to lib.rs
/// Add the template of struct Solution and test to the new file
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut no_rename = true;
    // parse args
    let repo_path = if args.len() == 1 {
        let script_path = Path::new(args.get(0).unwrap());
        script_path
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .canonicalize()
            .unwrap()
    } else if args.len() == 2 {
        Path::new(args.get(1).unwrap()).canonicalize().unwrap()
    } else {
        panic!("Invalid argument number. Expect 0 or 1 argument.");
    };
    let src_path = repo_path.join("src");
    println!("{}", src_path.to_str().unwrap());

    let lib_path = src_path.join("lib.rs");
    let lib_file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(&lib_path)
        .unwrap();

    // get all file names in src directory
    for filename in src_path.read_dir().unwrap() {
        let filename = filename.unwrap();
        let file_name = filename.file_name().into_string().unwrap();
        let rust_module_name = leetcode_name_to_rust_module_name(&file_name);
        match rust_module_name {
            Some(rust_module_name) => {
                no_rename = false;
                // Rename file name to meet rust module name rule
                let new_script_path = src_path.join(String::from(&rust_module_name) + ".rs");
                std::fs::rename(src_path.join(&file_name), &new_script_path).unwrap();
                println!("renamed {} to {}", &file_name, &rust_module_name);
                // 5.longest-palindromic-substring.rs
                // Add the file name to lib.rs
                write!(&lib_file, "pub mod {};\n", &rust_module_name).unwrap();
                println!("added {} to lib.rs", &rust_module_name);

                // Add the template of struct Solution and test to the new file
                let new_script_file = std::fs::OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&new_script_path)
                    .unwrap();
                write!(
                    &new_script_file,
                    "struct Solution;\npub fn main() {{assert_eq!();}}\n"
                )
                .unwrap();
            }
            None => (),
        };
    }

    if no_rename {
        println!("No file needs to be renamed.");
    }
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

fn leetcode_name_to_rust_module_name(leetcode_name: &str) -> Option<String> {
    // check with regex if old_name is {number}.{word}-{word}-{...}.rs
    // copilot knows how to do regex
    // let re = Regex::new(r"^(\d+)\.([a-z]+(?:-[a-z]+)*)\.rs$").unwrap();
    let test_re = Regex::new(r"^(\d+)\.([\w]+)(-[a-z]+)*(\.rs)?$").unwrap();
    if !test_re.is_match(leetcode_name) {
        return None;
    }

    let rust_module_name: String = leetcode_name
        .chars()
        .map(|x| match x {
            '.' | '-' => '_',
            _ => x,
        })
        .collect();
    Some(format!(
        "q{}",
        rust_module_name[0..rust_module_name.len() - 3].to_owned()
    ))
}
