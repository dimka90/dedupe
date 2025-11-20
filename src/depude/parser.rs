use clap::{Arg, Command};

struct Depude{
    dir_path: String
}
impl  Depude {
    pub fn  new(dir_path: &str) -> Self{
        Self { dir_path: dir_path.to_string()}
    }
}
pub fn parser(){

    let dir_path = Command::new("Depude")
                            .about("Checks for duplicate file")
                            .arg(
                                Arg::new("dir-name")
                                .short('d')
                                .long("dir-name")
                                .required(true)
                            )
                            .get_matches();

    
    println!("{dir_path:?}");
}