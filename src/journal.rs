#[derive(Serialize, Deserialize, Debug)]
struct Journal {
    name: String,
    path: String,
}

impl Journal {
    pub fn new(name: String) -> Journal {
        Journal {
            name: name,
            path: "$HOME\\.123j".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn new_returns_journal() {
        //let mut f = File::create("foo.txt").expect("could not create file for some reason");

        let pj = Journal::new("test");
        //assert!(fs::remove_file("foo.txt").is_ok());
    }

}
