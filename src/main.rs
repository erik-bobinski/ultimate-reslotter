use native_dialog::FileDialog;
use regex::Regex;
use std::{
    fs::{rename, File},
    io::{Read, Write},
    path::PathBuf,
    process,
};
use walkdir::WalkDir;

fn main() {
    let (c_slot, root) = gather_inputs();
    reslotter(c_slot, root);
}

// gather and validate proper mod path and c-slot
fn gather_inputs() -> (String, String) {
    let check_mark = "\x1b[32m✔\x1b[0m";
    let cross_mark = "\x1b[31m✘\x1b[0m";

    // take user-input of costume slot
    let c_slot: String = loop {
        // run the input
        let mut input = String::new();
        print!("Enter the costume slot to re-slot to (example: for c00, enter 00): c");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut input)
            .expect(format!("{} Failed to read input", cross_mark).as_str());

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
                println!("{} Make sure you enter integers!\n", cross_mark);
            }
        } else {
            println!("{} Only enter two integers for the slot!\n", cross_mark);
        }
    };
    // confirm c-slot target
    println!("{} Costume slot read as c{}", check_mark, c_slot);

    // take user-input of mod path
    let root: PathBuf = loop {
        let root: Result<Option<PathBuf>, native_dialog::Error> =
            FileDialog::new().set_location("~").show_open_single_dir();

        match root {
            Ok(Some(path)) => {
                break path;
            }
            Ok(None) => {
                println!("{} No mod folder selected", cross_mark);
                process::exit(-1); // exit if user closes file dialog
            }
            Err(e) => {
                eprintln!(
                    "{} Error occurred when selecting mod folder: {:?}",
                    cross_mark, e
                );
            }
        }
    };
    // confirm path in unix-style
    let root: String = root.to_string_lossy().to_string();
    println!(
        "{} Mod read from: {:?}",
        check_mark,
        root.replace("\\", "/")
    );

    (c_slot, root)
}

// parse and re-slot mod
fn reslotter(c_slot: String, root: String) {
    let check_mark = "\x1b[32m✔\x1b[0m";
    let cross_mark = "\x1b[31m✘\x1b[0m";

    // patterns to search for
    let c_pattern = Regex::new(r"c\d\d").unwrap();
    let ui_pattern = Regex::new(r"(chara_\d+_[a-zA-Z]+_)\d{2}(\.bntx)").unwrap();
    let config_pattern = Regex::new(r"(?i)^config\.json$").unwrap();

    // sort by descending depth => avoids overwriting dirs while within them
    let mut sorted_entries: Vec<_> = WalkDir::new(&root)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .collect();
    sorted_entries.sort_by(|a, b| b.depth().cmp(&a.depth()));

    // search each entry for patterns
    for entry in sorted_entries {
        let file_name = entry.file_name().to_string_lossy().to_string();

        // detect cXY pattern
        if c_pattern.is_match(&file_name) {
            let new_file_name = c_pattern.replace(&file_name, format!("c{}", c_slot));

            let current_path = entry.path();

            let new_path = current_path
                .parent()
                .unwrap()
                .join(new_file_name.to_string());

            if let Err(e) = rename(current_path, &new_path) {
                eprintln!(
                    "{} Failed to re-slot {:?} to {:?}, {}",
                    cross_mark, current_path, new_path, e
                );
            } else {
                println!(
                    "{} Re-slotted {:?} to {:?}",
                    check_mark,
                    current_path.file_name().unwrap(),
                    new_path.file_name().unwrap()
                );
            }
        }
        // detect ui file pattern
        else if ui_pattern.is_match(&file_name) {
            let new_file_name = ui_pattern.replace(&file_name, |caps: &regex::Captures| {
                format!("{}{}{}", &caps[1], c_slot, &caps[2])
            });

            let current_path = entry.path();

            let new_path = current_path
                .parent()
                .unwrap()
                .join(new_file_name.to_string());

            if let Err(e) = rename(current_path, &new_path) {
                eprintln!(
                    "{} Failed to re-slot {:?} to {:?}, {}",
                    cross_mark, current_path, new_path, e
                );
            } else {
                println!(
                    "{} Re-slotted {:?} to {:?}",
                    check_mark,
                    current_path.file_name().unwrap(),
                    new_path.file_name().unwrap()
                );
            }
        }
        // detect config file
        else if config_pattern.is_match(&file_name) {
            let mut file = File::open(entry.path()).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            // replace all cXY patterns in json string
            let contents_with_c = c_pattern.replace_all(&contents, format!("c{}", c_slot));

            // replace all ui patterns in json string
            let new_contents = ui_pattern
                .replace_all(&contents_with_c, |caps: &regex::Captures| {
                    format!("{}{}{}", &caps[1], c_slot, &caps[2])
                })
                .to_string();

            // write re-slotted contents to config file
            let mut file = File::create(entry.path()).unwrap();
            file.write_all(new_contents.as_bytes()).unwrap();

            println!(
                "{} Updated {:?}",
                check_mark,
                entry.path().file_name().unwrap()
            );
        }
    }
}
