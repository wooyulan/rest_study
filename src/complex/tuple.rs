pub fn test_tuple(){
    test_init();
}


fn test_init(){
    let tup : (i32,f64,u8) = (500,6.4,1);
    println!("{}",tup.1);
    println!("{}",tup.0);
    println!("{}",tup.2);

}