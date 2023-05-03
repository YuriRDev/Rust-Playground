use std::io;

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

pub enum Action {
    DELETE(i32),
    CREATE(Book),
    LIST,
    UPDATE(i32, Book),
}

pub fn get_action() {
    let mut input = String::new();

}
