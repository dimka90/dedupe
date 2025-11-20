use std::fs;
use std::path::Path;
use std::process;

pub fn get_dir_files(dir_path: &str) {
    let path = Path::new(dir_path);

    if !path.is_dir() {
        panic!("The file path {:?} is not a valid directory", path)
    }

    match fs::read_dir(path) {
        Ok(entries) => {
            println!("Files in this dir");
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let file_path = entry.path();
                        if file_path.is_file() {
                            println!("{}", file_path.display());
                        }
                    }
                    Err(e) => eprintln!("Error reading entry: {}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading directory '{}': {}", dir_path, e);
            process::exit(1);
        }
    }
}
