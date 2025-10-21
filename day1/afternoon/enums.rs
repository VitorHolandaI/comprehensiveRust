#[derive(Debug)]
enum Direction {
    Left,
    Right,
}
#[derive(Debug)]
enum PlayerMove {
    Pass,
    Run(Direction),
    Teleport { x: u32, u: u32 },
}

fn main() {
    let dir = Direction::Left;
    let player_move: PlayerMove = PlayerMove::Run(dir);
    //nao pega o valor diretao nao eh o mesmo de tupla de antes!
    println!("On this turn: {:?}",player_move);
}
