/// Represents a 2d point
#[derive(PartialEq, Eq, Clone)]
pub struct Point {
    pub x: u8,
    pub y: u8,
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

    /// Inverts the y coordinate of the point, so that it is mapped correctly to the grid
    pub fn invert(&mut self) {
        self.y = 9_u8.saturating_sub(self.y);
    }
}

impl TryFrom<String> for Point {
    type Error = &'static str;

    /// Converts a string in the format (Column Letter)(Row Number) to a point
    fn try_from(string: String) -> Result<Self, Self::Error> {
        let x_char = match string.chars().next() {
            Some(x_letter) => x_letter,
            None => return Err("You must guess!"),
        };

        let x = match letter_to_coordinate(x_char) {
            Some(x) => x,
            None => return Err("Character is non alphabetic!"),
        };

        let y: u8 = match string[1..].trim().parse() {
            Ok(y) => y,
            Err(_) => return Err("Failed to pass the Y portion of the coordinate, not numeric"),
        };

        let y: u8 = y.saturating_sub(1);

        if x >= crate::BOARD_SIZE || y >= crate::BOARD_SIZE {
            return Err("Guess outside of bounds!");
        }

        Ok(Point::new(x, y))
    }
}

/// Converts the given coordinate to a coordinate. This function does convert the letter to it's
/// Upper variant. Returns None if the letter is non alphabetic or fails to convert to a capital
///
/// Parameters
/// letter:   The letter to convert
fn letter_to_coordinate(letter: char) -> Option<u8> {
    if !letter.is_alphabetic() {
        return None;
    }

    let letter_upper = letter
        .to_uppercase()
        .collect::<Vec<char>>()
        .first()?
        .to_owned();

    let letter_as_u8 = letter_upper as u8;
    let letter_as_coordinate = letter_as_u8 - 65; // Taking away 65 as A in ASCII is 65

    Some(letter_as_coordinate)
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

    #[test]
    fn point_invert_work() {
        let mut point = Point::new(3, 2);
        point.invert();
        assert_eq!(point.y, 7);
    }

    #[test]
    fn letter_to_coordinate_works() {
        assert_eq!(letter_to_coordinate('A'), Some(0));
        assert_eq!(letter_to_coordinate('C'), Some(2));
        assert_eq!(letter_to_coordinate('c'), Some(2));
        assert_eq!(letter_to_coordinate('z'), Some(25));
        assert_eq!(letter_to_coordinate('Z'), Some(25));

        assert_eq!(letter_to_coordinate('-'), None);
        assert_eq!(letter_to_coordinate('4'), None);
        assert_eq!(letter_to_coordinate('^'), None);
    }
}
