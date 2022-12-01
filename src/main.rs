#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 参照を渡して呼び出し側に所有権を残す
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 関連関数はインスタンス化しなくても利用できる
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let square = Rectangle::square(100);

    println!("rect1 is {:?}", rect1);
    // println!("The area of rectangle is {} square pixels.", area(&rect1));
    println!("The area of rectangle is {} square pixels.", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("{:?}", square);
}
//
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
