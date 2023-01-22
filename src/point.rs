/// Represents a 2d point
#[derive(PartialEq, Eq)]
pub struct Point {
    x: u8,
    y: u8,
}

impl Point {
    /// Creates a new Point struct
    ///
    /// Parameters
    /// x:   The x coordinate of the point
    /// y:   The y coordinate of the point
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    /// Returns the x coordinate
    pub fn x(&self) -> u8 {
        self.x
    }

    /// Returns the y coordinate
    pub fn y(&self) -> u8 {
        self.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_works() {
        let point = Point::new(3, 2);
        assert_eq!(point.x, 3);
        assert_eq!(point.y, 2);
    }

    #[test]
    fn x_works() {
        let point = Point::new(3, 2);
        assert_eq!(point.x(), point.x);
    }

    #[test]
    fn y_works() {
        let point = Point::new(3, 2);
        assert_eq!(point.y(), point.y);
    }
}
