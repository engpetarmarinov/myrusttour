use std::convert::TryFrom;

pub fn main() {
    let x: u32 = 5;

    // shadowing x in the same scope with a different type
    let x: i32 = i32::try_from(x).unwrap() + 1;

    {
        // shadowing x in a different scope, same type i32
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
