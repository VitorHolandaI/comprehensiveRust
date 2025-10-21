#[derive(Debug)]
enum Direction {
    Left,
    Right,
}
struct Run(Direction);//struct unica com direcao

fn main() {
    let r = Run(Direction::Left);
    println!("{:?}", r.0);
}
