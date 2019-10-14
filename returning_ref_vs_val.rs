fn main() {
    let x = 10;
    let y = 20;
    println!("x = {}", bar(&x, &y));
}


// Lifetime specifier missing param when we return the reference
fn foo(x: &i32, y: &i32) -> &i32 {
    if 1 == 1 {
        x
    } else {
        y
    }
}

// When we return the value itself, compiler doesn't scream
fn bar(x: &i32, y: &i32) -> i32 {
    if 1 == 1 {
        *x
    } else {
        *y
    }
}
