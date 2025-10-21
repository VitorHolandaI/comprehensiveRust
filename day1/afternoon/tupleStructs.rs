struct Point(i32,i32);

// ofter used to singl;e field wrappers!!!
//
//
// stuct PoundsOfForce(f64);
// stuct Newtons(f64);
fn main() {
    let mut point = Point(2,2);
    println!("{} {}",point.0,point.1);
}
