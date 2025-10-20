fn fibo(n: i32) -> i32 {
    if n < 2 {
        return n;
    } else {
        println!("value n is {n}");
        return fibo(n - 1) + fibo(n - 2);
    }
}

fn main() {
    let n = 10;
    println!("fibo of {} is {}", n, fibo(n));
}
