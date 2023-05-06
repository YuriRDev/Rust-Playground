use lib_system::*;
fn main() {
    let mut lib = Library::new();

    while (true) {
        lib.get_action();
    }
}

fn list(vector: &Vec<Book>) {
    for book in vector {
        book.print();
    }
}
