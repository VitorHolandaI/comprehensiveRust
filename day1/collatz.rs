fn collatz_length(mut n: i32) -> u32 {
    let mut len = 0;
    loop {
        if n == 1 {
            len+=1;
            break;
        } else {
            if n % 2 == 0 {
                n = n / 2;
                dbg!(n);
                len += 1;
            } else {
                n = 3 * n + 1;
                dbg!(n);
                len += 1;
            }
        }
    }
    len
}

fn main() {
    println!("Length: {}", collatz_length(11)); // should be 15
}

