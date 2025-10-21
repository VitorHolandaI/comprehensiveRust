fn main() {
    let x_ref = {
        let x = 10;
        &x //vc retorna uma referencia porem esta acaba nao faz sentido !! 
    };

    dbg!(x_ref);
}
