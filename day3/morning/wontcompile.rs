fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let elem = &vec[2];
    vec.push(6);//borrow here tooo evita que modifique algo em uso
    dbg!(elem);
}
