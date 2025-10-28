//RefCell allows accessing and mutating a wrapped value
//by providing alternative types Ref and RefMut that
//emulate &T/&mut T without actually being Rust references.
//
use std::cell::RefCell;

fn main() {
    // Note that `cell` is NOT declared as mutable.
    let cell = RefCell::new(5);

    {
        let mut cell_ref = cell.borrow_mut();
        *cell_ref = 123;

        // This triggers an error at runtime.
        // let other = cell.borrow();
        // println!("{}", other);
    }

    println!("{cell:?}");
}

