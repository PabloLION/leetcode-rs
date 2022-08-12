use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_filename = String::from("1");

    let filename: &String = match args.get(1) {
        Some(filename) => filename,
        None => {
            println!("Using default filename: {}", default_filename);
            &default_filename
        }
    };
    let new_filename = new_file_name_for_rust_module(filename);
    cli_clipboard::set_contents(new_filename.to_owned()).unwrap();
    println!("`{}' is copied to clipboard", new_filename);
}

fn new_file_name_for_rust_module(old_name: &str) -> String {
    // TODO: check with regex if old_name is {number}.{word}-{word}-...

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
