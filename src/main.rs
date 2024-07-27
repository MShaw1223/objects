use std::fs;
use std::io::Write;

fn main() {
    let new_action = FileActions;
    new_action.create("input");
    println!("File created...");
    new_action.append("input");
    println!("File Edited...");
    new_action.overwrite("input", "This is editted");
    println!("File Overwritten");
    new_action.delete("input");
    println!("File Deleted...");
}

struct FileActions;

impl FileActions {
    fn create(&self, path: &str) {
        self.create_with_text(path, "Hello, World !");
    }
    fn create_with_text(&self, path: &str, text: &str) {
        let formatted_path: String = format!("{}.txt", path);
        match fs::File::create(&formatted_path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(text.as_bytes()) {
                    eprintln!("Failed to write message to file {}: {}", formatted_path, e);
                }
            }
            Err(e) => {
                eprintln!("Failed to create file {}: {}", formatted_path, e);
            }
        }
    }
    fn append(&self, input: &str) {
        self.append_with_text(input, "Default message due to no input");
    }
    fn append_with_text(&self, path: &str, text: &str) {
        let formatted_path: String = format!("{}.txt", path);
        let message = format!("\n{}", text);
        let byte_message = message.as_bytes();

        match fs::OpenOptions::new().append(true).open(formatted_path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(byte_message) {
                    eprintln!("Error appending to file {}: {}", path, e);
                }
            }
            Err(e) => {
                eprintln!("Failed to open file {}: {}", path, e);
            }
        }
    }
    fn delete(&self, input: &str) {
        let path: String = format!("{}.txt", input);
        if let Err(e) = fs::remove_file(&path) {
            eprintln!("Failed to delete file: {}", e);
        }
    }
    fn overwrite(&self, path: &str, text: &str) {
        let formatted_path: String = format!("{}.txt", path);
        let byte_message = text.as_bytes();
        if let Err(e) = fs::write(&formatted_path, byte_message) {
            eprintln!("Error writing to file {}: {}", formatted_path, e);
        }
    }
}
