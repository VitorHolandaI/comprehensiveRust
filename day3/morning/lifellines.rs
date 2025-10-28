struct Book {
    title: &str,
}

fn main() {
    let title = String::from("Rust Book"); // <-- lives for whole function
    let book = Book { title: &title };     // borrow is valid
    println!("{}", book.title);
} // both dropped safely

