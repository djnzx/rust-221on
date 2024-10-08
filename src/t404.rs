//1
#[test]
fn main1() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s: i32 = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
//2
#[test]
fn main2() {
    print();
}
// Replace i32 with another type
fn print(){
    println!("Success!");
}
//3
// Solve it in two ways
// DON'T let `println!` work
#[test]
fn main3() {
    never_return();

    println!("Failed!");
}
fn never_return() -> ! {
    // Implement this function, don't modify the fn signatures
    panic!()
}
//4
#[test]
fn main4() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };

    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    panic!()
}
//5
#[test]
fn main5() {
    // FILL in the blank
    let b = false;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}