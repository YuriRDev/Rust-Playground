use lib_system::*;
fn main() {
    let mut array: Vec<Book> = Vec::new();

    let mut a = Book::new(String::from("Nome"), String::from("Description"));

    get_action();

    list(&array);
}

fn list(vector: &Vec<Book>) {
    for book in vector {
        book.print();
    }
}
