//1
// Remove something to make it work
#[test]
fn test1() {
    let x: i32 = 5;
    let mut y = 5;
    y = x;
    let z:i32 = 10; // Type of z ?
    println!("Success!");
}
//2
#[test]
// Fill the blank
fn test2() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}
//3
#[test]
// Modify `assert_eq!` to make it work
fn test3() {
    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));

    println!("Success!");
}
// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())//i32
}
//4
#[test]
// Fill the blanks to make it work
fn test4() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
    println!("Success!");
}
//5
#[test]
// Fix errors and panics to make it work
fn test5() {
    let v1:u16 = 251_u16 + 8;
    let v2:i16 = i16::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2);
}
//6
#[test]
// Modify `assert!` to make it work
fn test6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;//1024 + 255 + 63 + 255
    assert!(v == 1597);

    println!("Success!");
}
//7
#[test]
// Fill the blank to make it work
fn test7() {
    let x: f64= 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z: f64 = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}
fn type_of1<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

//8
#[test]
fn test801() {
    assert!(0.1_f32 + 0.2_f32==0.3_f32);
    println!("Success!");
}
#[test]
fn test802() {
    assert!(0.1 as f32+0.2 as f32==0.3 as f32);
    println!("Success!");
}
//9
#[test]
fn test9() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }
    assert!(sum == -5);
    for c in 'a'..='z' {
        println!("{}",c as u8);
    }
}
//10
// Fill the blanks

use std::ops::{Range, RangeInclusive};
#[test]
fn test10() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}
//11
// Fill the blanks and fix the errors
#[test]
fn test11() {
    // Integer addition
    assert!(1u32 + 2u32 == 3u32);

    // Integer subtraction
    assert!(1i32 - 2i32 == -1i32);
    assert!(1i8 - 2i8 == -1);

    assert!(3 * 50 == 150);//i32

    assert!(9.6 as f32 / 3.2 as f32 == 3.0 as f32); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
