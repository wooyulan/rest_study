use core::slice;

pub fn test_array(){
    init__basic_array();
    init_no_basic_array();
    array_slice();
}


// 基础类型的数组
fn init__basic_array(){
    let a = [1,2,3,4,5];
    println!("{}",a[3]);
}

// 非基础类型的数组
fn init_no_basic_array(){
    // /**
    //  * 初始化报错:
    //  * 基础数据是在栈内copy
    //  * 非基础类型的数据是引用堆的指针
    //  * so 下面的初始化方式报错
    //  */
    // let error_array =[String::from("rust is good!");8];
    // println!("{:#?}", error_array);  


    // std::array::from_fn 循环初始化
    let array:[String;8] = std::array::from_fn(|_i| String::from("Rust"));
    println!("{:#?}", array); 
}


// 数组切片
fn array_slice(){
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // [1..3]  不包含
    // [1..=3] 包含
    // &[i32] 表示 类型地址
    let slice: &[i32] = &a[1..=3];
    println!("{:?}",slice)
}



