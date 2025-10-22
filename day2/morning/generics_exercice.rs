use std::cmp::Ordering;

// TODO: implement the `min` function used in the tests.
//#fn min<T:Ord> (val1: T  val2: T) -> T {
//#    match val1.cmp(&val2){
//#        Ordering::Less => val1,
//#        Ordering::Greater => val2,
//#        Ordering::Equal =>val1,
//#    }
//#
//#}
//also works""" top one
fn min<T:Ord> (val1: T , val2: T) -> T {
    if val1 > val2 {
        val2
    } else {
        val1
    }
}

#[test]
fn integers() {
    assert_eq!(min(0, 10), 0);
    assert_eq!(min(500, 123), 123);
}

#[test]
fn chars() {
    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');
}

#[test]
fn strings() {
    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");
}
