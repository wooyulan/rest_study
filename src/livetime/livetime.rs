pub fn test_live(){
   // test_example();
}



/* example1  start */
#[derive(Debug)]
struct Foo;

impl Foo {
    //借用了 &mut self， 
    fn mutate_and_share(&mut self) -> &Self {
        &*self   //不可变借用
    }
    // 不可变借用
    fn share(&self){}
}
fn test_example() {
    let mut foo = Foo;
    let loan = foo.mutate_and_share();
    // foo的生命周期到此结束!  代码编译报错
   // foo.share();
    println!("{:?}", loan);
}