use std::env;
use std::fs;

const IN_DIR: &str = "./in";
const OUT_DIR: &str = "./out";

fn main() {
    // Setup Directories For Processing
    extract_input_files();
    clear_out_directory();

    // Process the logs
    process_logs();
}

fn extract_input_files() {
    let in_dir_info = fs::read_dir(IN_DIR).unwrap();

    for entry in in_dir_info {
        let path = entry.unwrap().path();
        if let Some(ext) = path.extension() {
            if ext == "gz" {
                println!("Extracting File: {}", path.display())
            }
        }
    }
}

fn clear_out_directory() {
    let out_dir_info = fs::read_dir(OUT_DIR).unwrap();
    for entry in out_dir_info {
        let entry = entry.unwrap();
        if (entry.file_name() != ".gitignore") {
            fs::remove_file(entry.path()).expect("File Delection Failed");
        }
    }
}

fn process_logs() {
    let in_dir_info = fs::read_dir(IN_DIR).unwrap();

    // For each file parse it
    for entry in in_dir_info {
        parse_file_contents(&entry.unwrap());
    }
}

fn parse_file_contents(entry: &std::fs::DirEntry) {
    if entry.file_name() == ".gitignore" {
        return;
    }

    let path_name = entry.path().display().to_string();
    println!("Parsing File: {path_name}");

    // Get file contents
    // let contents = fs::read_to_string(path_name).unwrap();
    // println!("{contents} \n======================================")
}
