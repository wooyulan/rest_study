pub fn test_process_control(){
    test_if();
    test_for();
}



// 用 if 来赋值时，要保证每个分支返回的类型一样
fn test_if(){
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);
}

/**
 * for 元素 in 集合 {
 * // 使用元素干一些你懂我不懂的事情
 * }
 * 
 *  注: 1.使用 for 时我们往往使用集合的引用形式,不使用引用会影响所有权
 * for item in &container {
 * // ...
 * }
 *  
 * 2. 如果想在for循环中修改元素
 * for item in &mut collection {
 * // ...
 *  }
 * 
 */
fn test_for(){
    for i in 1..=5 {
        println!("{}", i);
    }
    test_for_index();
}

//循环获取索引
fn test_for_index(){
    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }

    // 用for循环控制执行次数
    for _ in 1..=2 {
        println!("hahah")
    }
}


