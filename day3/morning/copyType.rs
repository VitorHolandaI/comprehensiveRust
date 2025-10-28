#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

//pois implementa o copy trait entao por default vai copiar se possivel no caso
fn main() {
    let p1 = Point(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}
