use std::path::Path;
use std::fs;
pub fn get_dir_files(dir_path: &str) {
    let path = Path::new(dir_path);

    if !path.is_dir(){
        panic!("The file path {:?} is not a valid directory", path)
    }
    
}