fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

struct NotCloneable;

fn main() {
    let foo = String::from("foo");
    let pair = duplicate(foo);
    //let pair = duplicate(NotCloneable); test to see if it clones
    println!("{pair:?}");
}
