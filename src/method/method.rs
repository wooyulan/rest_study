pub fn test_method(){
    test_rectangle();
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height:u32,
}

// 为结构体 Rectangle 实现方法
impl Rectangle {

    // &self == self: &self的简写  表示对Rectangle的不可变借用
    // &mut self 表示可变借用
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 允许方法名和结构体字段一致
    fn width_1(&self) -> bool {
        self.width >0
    }
        
    
    pub fn new(width:u32,height:u32) -> Self {
        Rectangle { width, height }
    }

    // 构造器
    pub fn width(&self) ->u32 {
        return self.width;
    }

}

fn test_rectangle(){

    let rect1 = Rectangle {
        width:30,
        height: 50
    };
    let t1 = rect1.area();
    println!("{}",t1);


    if rect1.width_1() {
        println!("{}",rect1.width)
    }

    let rect2 = Rectangle::new(40, 50);
    println!("{}",rect2.width())
}