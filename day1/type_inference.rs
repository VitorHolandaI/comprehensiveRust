//jsut to say that rust will default integert to i32 and float to float 64 ! BUT HEY its not
//dynamic !
fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn main() {
    let x = 10;
    let y = 20;

    takes_u32(x);
//    takes_i8(y);// funny here will make it i8 if this stays u32 does not work
//
//    takes_u32(y); but if this one here stays it does ehhhe rust types borrows etc
    takes_u32(y);
}
