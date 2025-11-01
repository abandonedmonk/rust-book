// ========= USING AS FUNC =============
// pub fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

#[derive(Debug)] // Just above the struct which we want to debug
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    // Getters
    pub fn width(&self) -> bool {
        self.width > 0
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Constructor
    pub fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }
}
