use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

use flate2::read::GzDecoder;
use regex::Regex;

const IN_DIR: &str = "./in";
const OUT_DIR: &str = "./out";

fn main() {
    // Setup Directories For Processing
    extract_input_files();
    clear_out_directory();

    // Process the logs
    process_logs();
}

fn has_extension(path: &PathBuf, extension: &str) -> bool {
    if let Some(ext) = path.extension() {
        if ext == extension {
            return true;
        }
    }
    return false;
}

fn extract_input_files() {
    let in_dir_info = fs::read_dir(IN_DIR).unwrap();

    for entry in in_dir_info {
        let path = entry.unwrap().path();
        if has_extension(&path, "gz") {
            println!("Extracting File: {}", path.display());

            // Open file to extract
            let file = File::open(&path).unwrap();
            let mut decoder = GzDecoder::new(file);

            // Create output file for extraction
            let file_name = path.file_stem().unwrap().to_str().unwrap();
            let out_file_path = IN_DIR.to_owned() + "/" + file_name;
            let mut out_file = File::create(out_file_path).unwrap();

            // Write extracted data into file
            let mut buffer = [0; 4096];
            loop {
                match decoder.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => out_file.write_all(&buffer[..n]).unwrap(),
                    Err(e) => panic!("Error while decompressing file: {:?}", e),
                }
            }
        }
    }
}

fn clear_out_directory() {
    let out_dir_info = fs::read_dir(OUT_DIR).unwrap();
    for entry in out_dir_info {
        let entry = entry.unwrap();
        if entry.file_name() != ".gitignore" {
            fs::remove_file(entry.path()).expect("File Deletion Failed");
        }
    }
}

fn process_logs() {
    let in_dir_info = fs::read_dir(IN_DIR).unwrap();

    // For each file parse it
    for entry in in_dir_info {
        let entry = entry.unwrap();
        let path = entry.path();

        if has_extension(&path, "log") {
            let contents = parse_file_contents(&entry);
            let filtered = filter_log_messages(&contents);
            save_log(&entry, &filtered);
        }
    }
}

fn parse_file_contents(entry: &std::fs::DirEntry) -> String {
    let path = entry.path();
    let path_name = path.display().to_string();

    println!("Parsing File: {path_name}");

    // Get file contents
    let mut file = File::open(path_name).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let contents = String::from_utf8_lossy(&buffer).into_owned();
    // println!("{contents} \n======================================");
    return contents;
}

fn filter_log_messages(messages: &str) -> String {
    // Split log by timestamp
    let regex = Regex::new(r"(\[\d{2}:\d{2}:\d{2}\]\s+)").unwrap();

    // Only keep info messages from log
    let messages: Vec<&str> = regex
        .split(messages)
        .filter(|msg| is_info_msg(*msg))
        .collect();

    // Remove startup info from log
    let full_log = messages.join("");
    let split_pos = full_log.find("[Server thread/INFO]: Done");
    let log = match split_pos {
        Some(split_pos) => full_log.split_at(split_pos).1,
        None => &full_log,
    };

    return String::from(log);
}

fn is_info_msg(msg: &str) -> bool {
    return msg.contains("[Server thread/INFO]");
}

fn save_log(entry: &std::fs::DirEntry, log: &str) {
    let path = entry.path();
    let file_name = path.file_stem().unwrap().to_str().unwrap();
    let out_file_path = OUT_DIR.to_owned() + "/" + file_name + ".log";

    fs::write(out_file_path, log).unwrap();
}
