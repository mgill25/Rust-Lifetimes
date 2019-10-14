fn main() {
    let r;
    {
        let x = 10;         // x's lifetime starts here
        r = &x;             // x is borrowed via a reference
    }                       // x's lifetime ends here, x is dropped
    println!("r = {}", r);  // borrowed value doesn't live long enough!
}
