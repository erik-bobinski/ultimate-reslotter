// mod mod_directory;

// use mod_directory::*;
use native_dialog::FileDialog;
use std::{
    fs::{self, File},
    io::Write,
    process,
};
use walkdir::WalkDir;

fn main() {
    // take user-input of costume slot
    let c_slot = loop {
        // run the input
        let mut input = String::new();
        print!("Enter the costume slot to reslot to (example: for c00, enter 00): c");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // filter out whitespace
        let input: String = input.chars().filter(|c| !c.is_whitespace()).collect();

        // ensure proper slot indices within [0,99]
        let numbers: Vec<char> = input.chars().collect();
        if numbers.len() == 2 {
            // ensure each char can be casted to int
            let first: Option<u32> = numbers[0].to_digit(10);
            let second: Option<u32> = numbers[1].to_digit(10);
            if let (Some(_first), Some(_second)) = (first, second) {
                break input;
            } else {
                println!("Make sure you enter integers!\n")
            }
        } else {
            println!("Only enter two integers for the slot!\n")
        }
    };
    println!("\x1b[32m✔\x1b[0m Costume slot read as c{}", c_slot);

    // take user-input of mod path
    let root_path = loop {
        let root = FileDialog::new().set_location("~").show_open_single_dir();

        match root {
            Ok(Some(path)) => {
                break path;
            }
            Ok(None) => {
                println!("No mod folder selected");
                process::exit(-1); // exit if user closes file dialog
            }
            Err(e) => {
                eprintln!("Error occurred when selecting mod directory: {:?}", e);
            }
        }
    };
    // convert path to String in unix-style
    let root_str = root_path.to_string_lossy().to_string().replace("\\", "/");
    println!("\x1b[32m✔\x1b[0m Mod path read as: {:?}", root_str);

    // TODO: cXY pattern identification and renaming

    // rename example.txt to example_renamed.txt
    // let res: Result<(), std::io::Error> =
    //     fs::rename("./example/example.txt", "./example/example_renamed.txt");
    // if let Err(e) = &res {
    //     println!("error occurred: {}", e);
    // }

    // println!("calling print_directory on {}:", target_dir);
    // let res = print_directory(&target_dir);
    // println!("{:?}", res);
    // println!("------------------------------------------------------------");
}

// print every entry in a root directory
fn print_directory(target_dir: &str) {
    for entry in WalkDir::new(target_dir) {
        match entry {
            Ok(e) => println!("Found: {}", e.path().display()),
            Err(e) => println!("Error reading entry: {}", e),
        }
    }
}
