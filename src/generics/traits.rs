use std::fmt::{Display, Debug};

pub fn test_trait(){
    test();
}

// 定义接口  将 Summary 定义成了 pub 公开的。这样，如果他人想要使用我们的 Summary 特征，则可以引入到他们的包中，然后再进行实现。
pub trait Summary{
    // 定义特征
    fn summarize(&self) -> String;

    //定义一个默认实现
    fn summarize_default(&self) -> String {
        String::from("Read more...")
    }
}



pub struct Post {
    pub title: String, // 标题
    pub author: String, // 作者
    pub content: String, // 内容
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}



pub struct Weibo {
    pub username: String,
    pub content: String
}


impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }

    fn summarize_default(&self) -> String {
        format!("@{}", self.username)
    }

}



// 测试特征实现
fn test(){
    let post = Post {title:"Rust".to_string(),author:"Sunface".to_string(),content:"Rust棒极了!".to_string()};
    println!("{:?}",post.summarize());
    println!("{:?}",post.summarize_default());

    let weibo = Weibo{username: "sunface".to_string(),content: "好像微博没Tweet好用".to_string()};

    println!("{:?}",weibo.summarize());
    println!("{:?}",weibo.summarize_default());
}





// 使用特征作为函数参数  
// 实现了Summary特征 的 item 参数。
// 在函数体类 可以调整特征的方法
// 下面两种写法等价

pub fn notify(item: &impl Summary) {
    println!("Breaking news!{}",item.summarize())
}

pub fn notify_1<T: Summary>(item: &T){
    println!("Breaking news!{}",item.summarize())
}


// 多条件约束
// 除了让参数实现 Summary 特征外，还可以让参数实现 Display 特征以控制它的格式化输出
pub fn notify_more<T: Summary + Display>(item: &T){

}

// 当特征约束变多时候,可以使用where改进
fn some_function<T,U>(t:&T,u:&U) -> i32
     where T: Display +Clone,
           U: Clone +Debug
           {
            3+2
           }



// /**
//  *  参数是 Box<dyn Draw> 形式的特征对象，该特征对象是通过 Box::new(x) 的方式创建的
//  *  参数是 &dyn Draw 形式的特征对象，该特征对象是通过 &x 的方式创建的
//  * dyn 关键字只用在特征对象的类型声明上，在创建时无需使用 dyn
//  */



