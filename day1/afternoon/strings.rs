fn main() {
    // &str is a slice o utf-8 encoded bytes similar to &[u8]
    // String is a owned buffer of utf-8 encode bytes similar to Vec<t>
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Helo ");
    println!("s2: {s2}");

    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[2..9];

    println!("s3: {s3}");
}
