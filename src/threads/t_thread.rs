pub fn test_thread(){
    te_thread();
    thread_barrier();
}



use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

fn te_thread() {
    
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join，可以让当前线程阻塞，直到它等待的子线程的结束
    handle.join().unwrap();


    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

}



// 线程屏障
fn thread_barrier(){
    let mut handlers = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    for _ in 0..6 {
        let b = barrier.clone();
        handlers.push(thread::spawn(move || {
            println!("before wait");
            b.wait();
            println!("after wait");
        }));
    }
    
    for handler in handlers {
        handler.join().unwrap()
    }


}

