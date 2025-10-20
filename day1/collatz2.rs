fn collatz_length(mut n: i32) -> u32 {
    let mut len = 1;
    while n != 1 {
        n = match n % 2 {
            0 => n / 2,
            _ => 3 * n + 1,
        };
        len += 1;
    }
    len
}

fn main() {
  println!("Length: {}", collatz_length(11)); // 15


}
