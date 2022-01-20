fn main() {
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    let rect2 = Rectangle {
        width: 25,
        height: 5,
    };

    let rect3 = Rectangle::square(5);

    println!("The area of the rectangle is {} square pixels", rect.area());
    // dbg!(&rect);
    println!("The 2nd rect can fit inside: {}", rect.can_fit(&rect2));
    dbg!(rect3);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_fit(&self, other: &Rectangle) -> bool {
        other.width < self.width && other.height < self.height
    }

    fn square(dimension: u32) -> Rectangle {
        Rectangle {
            width: dimension,
            height: dimension,
        }
    }
}
