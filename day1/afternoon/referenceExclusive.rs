fn main(){
let mut point = (1,2);
let x_coord = &mut point.0; //bound onlyt to taht value! cannot be changed
*x_coord = 20;
println!("point: {point:?}");

}
