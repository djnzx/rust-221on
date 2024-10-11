//1
#[test]
fn main1() {
    // Use as many approaches as you can to make it work
    let x: String = String::from("Hello world");
    let y: String = x.clone();
    println!("{}, {}",x, y);
}
//2
// Don't modify code in main!
#[test]
// Don't modify code in main!
fn main2() {
    let s1: String = String::from("Hello world");
    let s2: String = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String{
    println!("{}", s);
    s
}
//3
#[test]
fn main3() {
    let s: String = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s: String = String::from("Hello world");
    // Convert String to Vec
    let _s = s.as_bytes();
    s
}
//4
// Fix the error without removing any code
#[test]
fn main4() {
    let s: String = String::from("Hello World");

    print_str(s.clone());

    println!("{}", s);
}

fn print_str(s: String) {
    println!("{}",s)
}
//5
// Don't use clone ,use copy instead
#[test]
fn main5() {
    let x:(i32, i32, (), &str) = (1, 2, (), "hello");
    let y:(i32, i32, (), &str) = x;
    println!("{:?}, {:?}", x, y);
}

//6
// make the necessary variable mutable
#[test]
fn main6() {
    let s: String = String::from("Hello ");

    let mut s1 = s;

    s1.push_str("World!");

    println!("Success!");
}

//7
#[test]
fn main7() {
    let x: Box<i32> = Box::new(5);

    let mut y: Box<i32> = Box::new(1);      // update this line, don't change other lines!

    *y = 4;

    assert_eq!(*x, 5);

    println!("Success!");
}

//приклад
#[test]
fn main0() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);
}

//8
#[test]
fn main8() {
    let t: (String, String) = (String::from("hello"), String::from("world"));

    let _s: String = t.0;

    // Modify this line only, don't use `_s`
    println!("{:?}", t.1);
}

//9
#[test]
fn main9() {
    let t: (String, String) = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (s1, s2) = t.clone();

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}
