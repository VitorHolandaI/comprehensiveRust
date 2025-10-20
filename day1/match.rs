fn main() {
    let val = 1;
    match val {
        1 => println!("one"),
        10 => println!("ten"),
        100 => println!("one hundred"),
        _ => {
            println!("else");
        }
    }

    let flag = true;
    let val = match flag { //here holding values as well the example of the if
        true => 1,
        false => 0,
    };
    println!("The value of the flag is {val}");
}
