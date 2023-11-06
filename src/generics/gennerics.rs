pub fn test_gennerics() {
    test();
    test_struct();
    test_area();
}

fn testLargeT<T:PartialOrd>(list: &[T])  -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}


fn test(){
    let vec = vec![1,2,3,4,5,6];
    let r= testLargeT(&vec);
    println!("{:?}",r)
}

// 结构体中定义泛型

// 
struct Point<T> {
    x:T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

}

fn test_struct(){
    let point = Point{x:3};
    println!("{}",point.x)
}



// 不同类型的泛型

struct Area<T,U> {  
    x:T,
    y:U,
}

// 双泛型的函数模版
impl<T,U> Area<T,U> {
    fn mixup<V,W>(self, other: Area<V,W>) -> Area<T,W>{
        Area {
            x:self.x,
            y:other.y,
        }
    }
}

fn test_area(){
    let a1 = Area{x:5,y:10.4};
    let a2 = Area{x:"hello",y:'c'};
    let p3 = a1.mixup(a2);
    println!("{}--{}",p3.x,p3.y)
}