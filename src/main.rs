mod depude;

use crate::depude::parser::parser;
fn main() {
    let dir_path = parser();

    
    println!("{dir_path:?}");
    println!("Hello, world!");
}
