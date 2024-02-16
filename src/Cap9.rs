
use std::fs::File;
use std::io::ErrorKind;

// fn main(){
//     a();
// }

// fn a(){
//     b()
// }

// fn b(){
//     let mut num = 22;
//     c(num)
// }

// fn c(num:i32){
//     if num == 22{
//         panic!("crash and burn");
//     }
// }

fn main(){
    let f = File::open("hello.txt");
    let f = match f{
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("cannot create file with error {:?}",e)
            }
            other_error=>{
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };


}