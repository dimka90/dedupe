use clap::{Arg, Command};

struct Depude {
    dir_path: String,
}
impl Depude {
    pub fn new(dir_path: &str) -> Self {
        Self {
            dir_path: dir_path.to_string(),
        }
    }
}
pub fn parser() -> Option<String> {
    let matches = Command::new("Depude")
        .about("Checks for duplicate file")
        .arg(
            Arg::new("dir-name")
                .short('d')
                .long("dir-name")
                .required(true),
        )
        .get_matches();

    if let Some(dir_path) = matches.get_one::<String>("dir-name") {
        return Some(dir_path.clone());
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dir_path() {
        let dir_path = parser();
    }
}
