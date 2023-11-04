pub fn test_match(){
    example();
    test_match_value();
    test_match_bind();
    test_matches();
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
        },
        _ => println!("West"),  // 通过将 _ 其放置于其他分支后，_ 将会匹配所有遗漏的值。
    };
}


// 使用 match 表达式赋值
enum IpAddr {
    Ipv4,
    Ipv6
 }

fn test_match_value(){
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
        },
    }
}

fn test_match_bind(){
    let t1 = Coin::Quarter(UsState::Alabama);
    let r = value_in_cents(t1);
    println!("{}",r);
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
    Bar
}

fn test_matches(){
    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    v.iter().filter(|x| matches!(x, MyEnum::Foo));
    println!("{:?}",v)
}