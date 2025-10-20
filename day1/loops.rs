fn main() {
    //while loop
    let mut val = 1;
    while val <= 10 {
        val = val + 1;
        println!("Val is {val}");
    }
    //for loop
    //
    //non inclusive
    for x in 1..5 {
        println!("x is {x}");
    }

    for element in [2, 4, 5, 6, 10] {
        println!("element is {element}");
    }
    //inclusive for
    for x in 1..=5 {
        println!("x is {x}")
    }

    //finaly the loop its like a  while true usable for server   that stay forever ....
    //
    //
    let mut i = 10;
    loop {
        i += 1;
        dbg!(i);
        if i > 100 {
            break;
        }
    }
}

