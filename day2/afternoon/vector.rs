
fn main(){
    let mut v1 = Vec::new();
    v1.push(42);
    v1.push(42);
    v1.push(42);
    v1.push(42);
    println!("v1: len = {}, capacity {}",v1.len(),v1.capacity());
    let getted = v1.get(0);
    println!("got  {}, ",getted.unwrap());


}


//To index the vector you use [ ], but they will panic if out of bounds.
//Alternatively, using get will return an Option. The pop function will remove the last element.
