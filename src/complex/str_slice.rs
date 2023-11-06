// 字符串与切片调用入口
pub fn test_str_slice() {
    greet("test".to_string());
    test_slice();
    test_slice_utf_8();
    test_str_2_addr();
    test_addr_2_str();
    test_str_operator();
}

// test
fn greet(name: String) {
    println!("Hello, {}!", name)
}
// test
fn greet_addr(name: &String) {
    println!("我接受&str, {}!", name)
}

// 测试切片
fn test_slice() {
    let s = String::from("hello slice");
    let t1 = &s[0..2];
    println!("t1:{}", t1);

    let len = s.len();
    let slice = &s[..];
    println!("{}", slice)
}

// 切片utf8
fn test_slice_utf_8() {
    for c in "中国人".chars() {
        println!("{}", c);
    }
    let s = "我在学习";

    let s1 = utf8_slice::slice(s, 0, 1);
    println!("s1:{}", s1)
}
// 类型转换
fn test_str_2_addr() {
    let s = String::from("我是一个String串");
    greet_addr(&s)
}

// 类型转化
fn test_addr_2_str() {
    let s = "我是一个&str串";
    greet(s.to_string());
}


// 字符串操作
fn test_str_operator() {
    let mut s = String::from("我是一个很长的字符串，接下来要开始对我进行增删改操作");
    println!("原始 s = {}",s);

    // 追加字符串
    s.push_str(",祝你好运!!!");
     println!("追加字符串 push_str() -> {}", s);

    // 插入字符串
    // /**
    //  * 一种中文是3个字节  3 6 9
    //  */
    s.insert_str(3, "eric");
    println!("插入字符串 insert_str() -> {}", s);

}