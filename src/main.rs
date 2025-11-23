mod depude;
use crate::depude::scanner::get_dir_files;
use crate::depude::parser::parser;
fn main() {
    let dir_path = parser();
    get_dir_files(&dir_path.unwrap());
//     println!("{dir_path:?}");
//     println!("Hello, world!");
// }
}
