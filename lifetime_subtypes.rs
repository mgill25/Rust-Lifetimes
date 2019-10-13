// Rust lifetimes

fn main() {
    let a : i32 = 10;
    {
        let b : i32 = 20;
        let c : i32 = *max(&a, &b);
        println!("max of the two = {:?}", c);
    }
    
}

fn max<'a: 'b, 'b>(x: &'a i32, y: &'b i32) -> &'b i32 {
    println!("x = {}", x);      // seems like for reference, we can use x and &x both
    println!("&x = {}", &x);
    if *x > *y {
        return &x
    } 
    return &y
}
