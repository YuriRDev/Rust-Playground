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

        println!("Book title: ");
        io::stdin()
            .read_line(&mut name)
            .expect("Could not read the name!");

        println!("Book Description: ");
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
    pub fn get_input(&mut self) {
        let mut input = String::new();

        // Get input from CLI
        io::stdin()
            .read_line(&mut input)
            .expect("[error] Expected an input");

        // Sets a enum value to the query
        let action = match input.as_str().trim() {
            "CREATE" => Action::CREATE,
            "UPDATE" => Action::UPDATE,
            "DELETE" => Action::DELETE,
            "LIST" => Action::LIST,
            _ => {
                println!("[error] Expected a query. Received: {}", input);
                Action::NIL
            }
        };

        // Runs enum_query
        self.run_action(action);
    }

    fn run_action(&mut self, action: Action) {
        match action {
            Action::CREATE => {
                self.books.push(Book::new_verbose());
            }
            Action::DELETE => {}
            Action::LIST => {
                self.list_books();
            }
            Action::UPDATE => {}
            _ => {}
        }
    }

    fn list_books(&self) {
        println!("Total of {} books", &self.books.len());
        for book in &self.books {
            book.print();
        }
    }
}

enum Action {
    DELETE,
    CREATE,
    LIST,
    UPDATE,
    NIL,
}
