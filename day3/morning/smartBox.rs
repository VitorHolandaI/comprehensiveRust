#[derive(Debug)]
enum List<T> {
    /// A non-empty list: first element and the rest of the list.
    Element(T, Box<List<T>>),
    /// An empty list.
    Nil,
}

fn main() {
    let list: List<i32> = List::Element(1, Box::new(List::Element(2, Box::new(List::Nil))));
    println!("{list:?}");
}

//If Box was not used and we attempted to embed a List directly into the List, the compiler would not be able to compute a fixed size for the struct in memory (the List would be of infinite size).
//
//Box solves this problem as it has the same size as a regular pointer and just points at the next element of the List in the heap.
