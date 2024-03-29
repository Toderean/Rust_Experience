pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height:u32,
}

impl Rectangle {
    fn can_hold(&self,other:&Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle{
            width: 8,
            height: 7,
        };
        let smaller = Rectangle{
            width: 1,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }


    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn failing_test(){
    //     panic!("Test failed!!!");
    // }


}
