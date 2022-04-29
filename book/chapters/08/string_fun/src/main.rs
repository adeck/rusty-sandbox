fn main() {
    // equivalent to String::from("hello, world")
    let s = "hello, world".to_string();
    let s2 = "!".to_string();
    // this invalidates `s`, because it's a move operation from the perspective of s
    // also worth noting, the second argument has to be a &str, so
    // literals work just fine, and String args must have a `&` attached.
    let s3 = s + &s2;
    let s = s3 + "\n";
    println!("{}", s);
}
