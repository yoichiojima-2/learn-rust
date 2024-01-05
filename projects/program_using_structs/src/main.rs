fn main() {
    version1();
    version2();
    version3();
    version4();
}

fn version1() {
    fn main() {
        let width = 30;
        let height = 50;

        println!(
            "the area of the rectangle is {} square pixels.",
            area(width, height)
        );
    }

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
    main();
}

fn version2() {
    fn main() {
        let rectl = (30, 50);

        println!(
            "the area of the rectangle is {} square pixels.",
            area(rectl)
        );
    }

    fn area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }
    main();
}

fn version3() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn main() {
        let rectl = Rectangle {
            width: 30,
            height: 50,
        };
        println!(
            "the area of the rectangle is {} square pixels.",
            area(rectl)
        );
    }

    fn area(rectangle: Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
    main();
}

fn version4() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn main() {
        let rectl = Rectangle {
            width: 30,
            height: 50,
        };

        println!("rectl is {:?}", rectl);
        println!(
            "the area of the rectangle is {} square pixels.",
            area(rectl)
        );
    }

    fn area(rectangle: Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    main();
}
