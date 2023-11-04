pub fn test_match() {
    example();
    test_match_value();
    test_match_bind();
    test_matches();
    test_while_let();
    match_name_var();
}

enum Direction {
    East,
    West,
    North,
    South,
}

// match 跟其他语言中的 switch 非常像，_ 类似于 switch 中的 default。
fn example() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        }
        _ => println!("West"), // 通过将 _ 其放置于其他分支后，_ 将会匹配所有遗漏的值。
    };
}

// 使用 match 表达式赋值
enum IpAddr {
    Ipv4,
    Ipv6,
}

fn test_match_value() {
    let ip1 = IpAddr::Ipv6;
    let ip_str = match ip1 {
        IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };

    println!("{}", ip_str);
}

// 模式绑定
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // 25美分硬币
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn test_match_bind() {
    let t1 = Coin::Quarter(UsState::Alabama);
    let r = value_in_cents(t1);
    println!("{}", r);
}

/**
 *  只想要对 Some(3) 模式进行匹配, 不想处理任何其他 Some<u8> 值或 None 值;
 */

//  let v = Some(3u8);
//  match v {
//      Some(3) => println!("three"),
//      _ => (),

#[derive(Debug)]
enum MyEnum {
    Foo,
    Bar,
}

fn test_matches() {
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    v.iter().filter(|x| matches!(x, MyEnum::Foo));
    println!("{:?}", v);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}



// while let 循环
fn test_while_let() {
    let mut stack = Vec::new();
    // 向数组中插入元素
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // 行stack 弹出所有元素
    while let Some(top) = stack.pop() {
        println!("{}",top)
    } 


    let v = vec!['a','b','c'];
    for(index,value) in v.iter().enumerate() {
        println!("{} is at index {}",value,index)
    }

}



// 匹配命名变量
fn match_name_var(){
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // some(50) 没有匹配到x中定义的值 所以y是一个新的变量，并不与y=10有直接关系
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);
}