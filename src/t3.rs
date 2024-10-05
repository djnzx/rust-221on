// Fix the error below with lest amount of modification to the code
//1
#[test]
fn main1() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}

//2
#[test]
// Fill the blanks in the code to make it compile
fn main2() {
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}
//3
#[test]
//Fix the error below with last amount of modification
fn main3() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
}
//4
#[test]
// Fix the error with the use of define_x
fn main4() {
    define_x();
}
fn define_x() {
    let x = "hello";
    println!("{}, world", x);
}
//5
#[test]
// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn main5() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);  // Внутри блока `x` равно 12
    }

    assert_eq!(x, 5);  // После блока `x` снова равен 5

    let x = 42;
    println!("{}", x); // Prints "42".
}
//6
#[test]
// Remove a line in the code to make it compile
fn main6() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    x = x + 3;
    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!";
    println!("Success!");
}
//7.1
#[test]
fn main7() {
    let _x = 1;
}
// Warning: unused variable: `x`
//7.2
#[test]
#[allow(unused_variables)]
fn main702() {
    let x = 1;
}

//8
#[test]
fn main8() {
    let (mut x, y) = (1, 2);
    x += 2;
    assert_eq!(x, 3);
    assert_eq!(y, 2);
    println!("Success!");
}
//9
#[test]
fn main9() {
    //let (x, y);
    //(x,..) = (3, 4);
    //[.., y] = [1, 2];
    // Fill the blank to make the code work
    //assert_eq!([x,y], [3, 2]);

    println!("Success!");
}

