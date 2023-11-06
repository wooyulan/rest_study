pub fn test_trait(){
    test();
}

// 定义接口
trait Summary{
    fn summarize(&self) -> String;
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

fn test(){
    let post = Post {title:"Rust".to_string(),author:"Sunface".to_string(),content:"Rust棒极了!".to_string()};
    println!("{:?}",post.summarize())
}