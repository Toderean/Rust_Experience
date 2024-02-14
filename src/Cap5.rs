
#[derive(Debug)]



struct User{
    active:bool,
    username: String,
    email: String,
    sign_in_count:u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// struct User {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }
// 1 ~ struct User<'a> {
//     2 |     active: bool,
//     3 ~     username: &'a str,
// fn main() {
//     let user1 = User {
//         active: true,
//         username: "someusername123",
//         email: "someone@example.com",
//         sign_in_count: 1,
//     };
// }


fn main(){
    // let mut test = String::from("hello world!");
    // let bytes = test.as_bytes();
    
    // for (i,&item) in bytes.iter().enumerate(){
    //     println!(" {} {} ", i,item);
    // }

    // println!("function {}", first_word(&mut test));


    // let s = String::from("hello world");

    // let hello = &s[0..5];
    // let world = &s[6..11];

    let user1 = User{
        active: true,
        username: String::from("Toderean"),
        email: String::from("tode@email.com"),
        sign_in_count: 1,
    };


    println!("user in main {:?}", user1);
    println!("user from function {:?}", create_user(String::from("email@email.com"), String::from("Marcel")));

    dbg!(&user1);
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );

}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn create_user(email: String, username:String) -> User{
    User{
        active:true,
        username: username,
        email:email,
        sign_in_count:0
    }
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }