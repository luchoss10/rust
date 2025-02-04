fn _area(width: u32, height: u32) -> u32 {
    width * height
}

fn _area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
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

    fn width(&self) -> bool{
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
       Self {
           width: size,
           height: size,
       }
    }
}

fn main() {
    //let width1 = 30;
    //let height1 = 50;

    //let rect1 = (30, 50);

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect2)
    );

    println!("rect2 is {:?}", rect2);

    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect3);

    println!(
        "The area of the rectangle is {} square pixels. (method)",
        rect3.area()
    );
    
    if rect3.width() {
        println!(
            "The rectangle has a nonezero width; it is {}", rect3.width
            )
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width : 10,
        height : 40,
    };

    let rect3 = Rectangle {
        width : 60,
        height : 45,
    };
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sqr = Rectangle::square(3);

    println!("A square!! width: {}, height {}.", sqr.width, sqr.height); 
    
}
