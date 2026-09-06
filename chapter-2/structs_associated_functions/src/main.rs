struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // Intentional type mismatch for teaching purposes.
    let sq = Rectangle::square("3");
    println!("The area of the square is {} square pixels.", sq.area());
}
