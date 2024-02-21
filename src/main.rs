// use std::thread;
// use std::time::Duration;
// fn main(){
//     // let handle = thread::spawn(|| {
//     //     for i in 1..10{
//     //         println!("spawned thread number  = {}", i);
//     //         thread::sleep(Duration::from_secs(1));
//     //     }
//     // });
//     // handle.join().unwrap();
//     // for i in 1..5{
//     //     println!("main thread number = {}",i);
//     //     thread::sleep(Duration::from_secs(1));
//     // }

    
//     let v = vec![1, 2, 3];

//     let handle = thread::spawn(move || {
//         println!("Here's a vector: {:?}", v);
//     });

//     handle.join().unwrap();
// }

use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;
use std::sync::Mutex;
use std::rc::Rc;

fn main(){
    // let (tx,rx)  = mpsc::channel();
    // let tx2 = tx.clone();

    // thread::spawn(move || {
    //     let msgs  = vec![
    //         String::from("Hello"),
    //         String::from("brothers"),
    //         String::from("and"),
    //         String::from("sisters"),
    //     ];
    //     for val in msgs{
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // thread::spawn(move || {
    //     let msgs  = vec![
    //         String::from("inca"),
    //         String::from("cateva"),
    //         String::from("mesage"),
    //         String::from("bai"),
    //     ];
    //     for val in msgs{
    //         tx2.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // // let received = rx.recv().unwrap();
    // for received in rx{
    //     println!("{}",received);
    // }

    let m = Rc::new(Mutex::new(5));
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m :{:?}",m);

    let counter = Arc::new(Mutex::new(0));
    let mut handless=  vec![];

    for _ in  0..10 {
        let counter2 = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter2.lock().unwrap();
            *num += 1;
        });
        handless.push(handle);
    }

    for handle in handless{
        handle.join().unwrap();
    }

    println!("Result {} ",*counter.lock().unwrap());

}