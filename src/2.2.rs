// Fill the blanks in the code to make it compile
fn main() {
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}
//3
//Fix the error below with last amount of modification
fn main() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
}
//4
// Fix the error with the use of define_x
fn main() {
    define_x();
}

fn define_x() {
    let x = "hello";
    println!("{}, world", x);
}
