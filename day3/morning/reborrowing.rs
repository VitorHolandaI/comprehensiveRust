//mesmo exemplo mas com reborrowing bem legal
fn main() {
    let mut a: i32 = 10;
    let b = &mut a;

    {
        // Instead of borrowing `a` directly, reborrow
        // through `b`, effectively giving us two mutable
        // references.
        let c = &mut *b;
        *c = 20;
    }

    println!("b: {b}");
    println!("a: {a}");
}
