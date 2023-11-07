pub fn test_format() {
    test_print();
    test_format_prit();
}

fn test_print() {
    println!("Hello"); // => "Hello"
    println!("Hello, {}!", "world"); // => "Hello, world!"
    println!("The number is {}", 1); // => "The number is 1"
    println!("{:?}", (3, 4)); // => "(3, 4)"
    println!("{value}", value = 4); // => "4"
    println!("{} {}", 1, 2); // => "1 2"
    println!("{:04}", 42); // => "0042" with leading zeros
}

/**
 *  {} 用于输出
 *  {:?} 用户debug
 */

 #[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

fn test_format_prit() {
    let i = 3.1415926;
    let s = String::from("hello");
    let v = vec![1, 2, 3];
    let p = Person{name: "sunface".to_string(), age: 18};
    println!("{:?}, {:?}, {:?}, {:?}", i, s, v, p);
}