fn main() {
    func1();
}

fn func1() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            // &self is the same as self: &Rectangle
            self.width * self.height
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
        fn square(size: u32) -> Rectangle {
            // call this function with Rectangle::square(size)
            Rectangle { width: size, height: size }
        }
    }

    fn main() {
        let rect1 = Rectangle { width: 30, height: 50  };
        let rect2 = Rectangle {width: 10, height: 40 };
        let rect3 = Rectangle {width: 70, height: 45 };

        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
        println!("The area of rect1 is {} square pixels.", rect1.area());

        let sq = Rectangle::square(3);
        println!("The area of sq is {} square pixels.", sq.area());
    }

    main();
}
