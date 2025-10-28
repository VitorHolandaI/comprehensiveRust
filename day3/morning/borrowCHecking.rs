fn main() {
    let mut a = 10;
    let b = &mut a;

    //dbg!(b);
    {
        let c = &mut a;
        *c = 20;
    }

    dbg!(a);
    dbg!(b);
}
