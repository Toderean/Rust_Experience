// use std::io::{self, Read};

//4.3 se continua pe Cap5

//can't have multiple mutable reference
// let mut s = String::from("hello");
// let r1 = &mut s;
// let r2 = &mut s;
// println!("{}, {}", r1, r2);


//but having references to others is permited
// let mut s = String::from("hello");
// let r1 = &s; // no problem
// let r2 = &s; // no problem
// let r3 = &mut s; // BIG PROBLEM
// println!("{}, {}, and {}", r1, r2, r3);

fn main() {
    let x :[i32; 5] = [200,0,120,146,23];
    for &index in &x{
        let result:i32 = convert_to_celsius(index);
        println!("result {} is : {}", index, result);
    }
    println!("the {} member of fibonacci sequence is {}", 19,fibonacci(2));

    let mut s = String::from("hello");

    change(&mut s);
    println!("{}",s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn convert_to_celsius(x:i32) -> i32{
    (x-32) * 5/9
}

fn fibonacci(index: i32) -> i32 {
    if index == 0 {
        0
    } else if index == 1 {
        1
    } else {
        fibonacci(index - 1) + fibonacci(index - 2)
    }
}

