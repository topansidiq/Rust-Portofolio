fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test(){
    println!("Hello, Test!");
}
#[test]
fn test_variable(){
    let name = "Topan Sidiq";
    println!("Hello, {}", name);
}
#[test]
fn test_mutable(){
    let mut x = 5;
    println!("x is {}", x);
    x = 6;
    println!("x is {}", x);
}

#[test]
fn static_typing(){
    let x = 5;
    println!("x is {}", x);
    // x = "Topan"; Error
}

#[test]
fn shadowing(){
    let name = "Topan Sidiq";
    println!("Hello, {}", name);

    let name = "Salsabila Agustin";
    println!("Hello, {}", name);
}

// This is comment

/*
    This is multiple line comment
 */

#[test]
fn explicit(){
    let x = 5;
    let y = 5;
    println!("x is {}", x);
    println!("y is {}", y);
    println!("x + y = {} + {}\n\t  = {}", x, y, x + y);
}

#[test]
fn number(){
    let a = 10;
    println!("a is {}", a);
    let b = 10.5;
    println!("b is {}", b);

    println!("a + b = {} + {}", a, b);
}