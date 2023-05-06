use std::io::{self, stdout};

pub struct Book {
    name: String,
    description: String,
}

impl Book {
    pub fn new(name: String, description: String) -> Book {
        Book { name, description }
    }

    pub fn new_verbose() -> Book {
        let mut name = String::new();
        let mut description = String::new();

        io::stdin()
            .read_line(&mut name)
            .expect("Could not read the name!");

        io::stdin()
            .read_line(&mut description)
            .expect("Could not read the description!");

        Book {
            name: name.trim().to_string(),
            description: description.trim().to_string(),
        }
    }
}

impl Book {
    pub fn print(&self) {
        println!("{}", self.name);
        println!("{}", self.description)
    }
}

pub struct Library {
    books: Vec<Book>,
}

impl Library {
    pub fn new() -> Library {
        Library { books: Vec::new() }
    }
}
impl Library {
    pub fn get_action(&self) {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("[error] Expected an input");

        match input.as_str().trim() {
            "CREATE" => {
                println!("CREATE!")
            }
            "UPDATE" => {
                println!("UPDATE!")
            }
            "DELETE" => {
                println!("DELETE!")
            }
            "LIST" => {
                println!("LIST!")
            }
            _ => {
                eprintln!(
                    "[error] Expected CREATE|UPDATE|DELETE|LIST. Received: {}",
                    input
                );
            }
        }
    }

    fn run_action(&self) {}
}

enum Action {
    DELETE(i32),
    CREATE(Book),
    LIST,
    UPDATE(i32, Book),
}

pub fn get_action() {}
