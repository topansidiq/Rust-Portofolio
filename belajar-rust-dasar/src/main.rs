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

#[test]
fn number_conversion(){
    let a: i8 = 10;
    println!("a is {}", a);

    let b: i16 = a as i16;
    println!("b is {}", b);

    let c: i32 = b as i32;
    println!("c is {}", c);

    let d: i64 = 123123123;
    let e: i8 = d as i8;
    println!("e is {}", e);
}

#[test]
fn numeric_operator(){
    let a = 10;
    let b = 90;
    let c = a + b;
    println!("c is {}", c);
    println!("{}", a + c)
}

#[test]
fn augmented_assignment(){
    let mut a = 10;
    println!("a is {}", a);

    a += 10;
    println!("a is {}", a);

    a *= 2;
    println!("a is {}", a);
}

#[test]
fn boolean(){
    let a = true;
    let b: bool = a;
    println!("a is {}", a);
    println!("b is {}", b);
}

#[test]
fn comparison(){
    let a = 10;
    let b = 20;
    let result: bool = a > b;

    println!("Result is {}", result);
}

#[test]
fn boolean_operator(){
    let a = 20 > 10 && 8 == 7;
    println!("a is {}", a);

    let b = -20 == (-10 - 10) || false;
    println!("b is {}", b);

    let c = !a;
    println!("c is {}", c);

    let presenting = 80;
    let final_exam = 90;

    let past = presenting > 75 || final_exam > 80;
    println!("Is past? {}", past);
}

#[test]
fn character(){
    let chat1: char = 'a';
    let chat2: char = 'b';
    let chat3: char = 'c';

    println!("chat 1 is {}", chat1);
    println!("chat 2 is {}", chat2);
    println!("chat 3 is {}", chat3);
}

#[test]
fn tuple(){
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", tup);

    let a = tup.0;
    let b = tup.1;
    let c = tup.2;

    println!("a = {}, b = {}, c = {}", a, b, c);

    let (e, f, _) = tup;
    println!("e is {}, f is {}", e, f);
}