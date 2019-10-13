#![allow(dead_code)]
struct ComplexRefs<'a> {
    int_ref: &'a i32,
    unsigned_ref: &'a u32,
    str_ref: &'a str
}

fn main() {
    // initialize 3 integers
    let a : i32 = -423;
    let c : i32 = 2321;
    let d : u32 = 5;
    let str_ref : &str = "hahaha";
    
    let cr : ComplexRefs = ComplexRefs {
        int_ref: &a,
        unsigned_ref: &d,
        str_ref: str_ref
    };
    
    let e;
    {
        let r = cr;
        e = three_refs(&a, &c, &r);
    }
    
    println!("{}", e);
}

// I would like to return a reference now
// Compiler tells me that it expects a lifetime signature!
// Error message is very clear:
//    = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `a`, `_c`, or one of `cr`'s 2 lifetimes
fn three_refs<'a, 'b>(a: &'a i32, _c: &'a i32, cr: &'b ComplexRefs<'a>) -> &'a i32 {
    if *a > 5 {
        cr.int_ref
    } else {
        a
    }
}
