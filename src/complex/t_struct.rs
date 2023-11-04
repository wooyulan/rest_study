pub fn test_struct() {
    create_struct();
    create_tuple_struct();
}

//  #[derive(Debug)] 对结构体进行了标记，这样才能使用 println!("{:?}", s); 的方式对其进行打印输出
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


/**
 * 1.初始化实例时，每个字段都需要进行初始化
 * 2.初始化时的字段顺序不需要和结构体定义时的顺序一致
 * 
 *  !!!把结构体中具有所有权的字段转移出去后，将无法再访问该字段，但是可以正常访问其它的字段
 */
fn create_struct() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{:?}",user1);

    let user2 = build_user("wooyulan@163.com".to_string(), "wooyulan".to_string());
    println!("{:?}",user2);

    let user3 = User {
        email : String::from("mr.xinyi@yahoo.com"),
        ..user2
    };
    // 当结构体较大时，我们可能希望能够有更好的输出表现，此时可以使用 {:#?} 来替代 {:?}，输出如下:
    println!("{:#?}",user3);

    //user2 的部分字段所有权被转移到 user3 中： username 字段发生了所有权转移，作为结果，user1 无法再被使用。其他两个字段是基本类型，进行了copy
    println!("{:?}",user2.sign_in_count);
    
    // dbg! 输出到标准错误输出 stderr，而 println! 输出到标准输出 stdout。
    dbg!(&user3);

}

// 初始化
fn build_user(email:String,username:String) -> User {
     User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


// 元祖结构体
struct Color(i32,i32,i32);

fn create_tuple_struct(){
    let black = Color(255,255,255);
    println!("{}",black.2)
}



