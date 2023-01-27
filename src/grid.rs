use crate::point::Point;

// Grid for representing a
pub struct Grid {
    width: u8,
    height: u8,
}

impl Grid {
    /// Constructs a new Grid struct with the given width and height
    ///
    /// Parameters
    /// width:    The width of the grid
    /// height:   The height of the grid
    pub fn new(width: u8, height: u8) -> Self {
        Self { width, height }
    }

    /// Returns a string containing a grid
    pub fn write_grid(&self, hits: &[Point], misses: &[Point]) -> String {
        let mut grid = String::new();

        let end_of_column = self.height.saturating_sub(1);
        let end_of_row = self.width.saturating_sub(1);

        for y in 0..self.height {
            if y == 0 {
                for x in 0..self.width {
                    grid.push_str(self.next_row_segment(x, end_of_row, "   ╭───┬", "───┬", "───╮"));
                }
                grid.push('\n');
            }

            grid.push_str(&self.row_number(y));

            for x in 0..self.width {
                if misses.contains(&Point::new(x, y)) {
                    grid.push_str(self.next_row_segment(x, end_of_row, "│ M │", " M │", " M │"));
                } else if hits.contains(&Point::new(x, y)) { 
                    grid.push_str(self.next_row_segment(x, end_of_row, "│ X │", " X │", " X │"));
                } else {
                    grid.push_str(self.next_row_segment(x, end_of_row, "│   │", "   │", "   │"));
                }
            }

            grid.push('\n');

            for x in 0..self.width {
                if y == end_of_column {
                    grid.push_str(self.next_row_segment(x, end_of_row, "   ╰───┴", "───┴", "───╯"));
                } else {
                    grid.push_str(self.next_row_segment(x, end_of_row, "   ├───┼", "───┼", "───┤"));
                }
            }

            grid.push('\n');
        }
        grid.push_str(&self.column_letter_row());
        grid.push('\n');

        grid
    }

    /// Determines if the next row segment should be a beginning, middle, or end, and returns the
    /// appropriate &str
    ///
    /// Parameters
    /// x:                  The current x value
    /// max_x:              The x value of an end segment
    /// beginning_of_row:   The string to return if the segment is a beginning
    /// middle_of_row:      The string to return if the segment is a middle
    /// end_of_row:         The string to return if the segment is an end
    fn next_row_segment(
        &self,
        x: u8,
        max_x: u8,
        beginning_of_row: &'static str,
        middle_of_row: &'static str,
        end_of_row: &'static str,
    ) -> &'static str {
        if x == 0 {
            beginning_of_row
        } else if x == max_x {
            end_of_row
        } else {
            middle_of_row
        }
    }

    /// Returns the current row's number properly formatted, with two spaces after for a one digit
    /// number and two spaces after a a two digit number.
    ///
    /// Parameters
    /// y:   The current row
    fn row_number(&self, y: u8) -> String {
        let axis_number = self.height - y;

        let two_digit_number = axis_number > 9;

        if two_digit_number {
            return format!("{} ", axis_number);
        }

        format!(" {} ", axis_number)
    }

    /// Returns the letter labels for the columns as a singular row
    fn column_letter_row(&self) -> String {
        let mut row = String::from("     ");

        let mut current_letter_as_u8: u8 = 65;

        for _ in 0..self.width {
            let current_letter = current_letter_as_u8 as char;
            row.push_str(&format!("{}   ", current_letter));
            current_letter_as_u8 += 1;
        }

        row
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Checks if the constructor creates a Grid struct with the right width and height
    fn constructor_works() {
        let width = 4;
        let height = 4;

        let grid = Grid::new(width, height);

        assert_eq!(grid.height, height);
        assert_eq!(grid.width, width);
    }

    #[test]
    fn row_segment_works() {
        let grid = Grid::new(4, 4);

        let mut x = 0;
        let x_max = 3;

        assert_eq!(grid.next_row_segment(x, x_max, "1", "2", "3"), "1");

        x = 1;
        assert_eq!(grid.next_row_segment(x, x_max, "1", "2", "3"), "2");

        x = x_max;
        assert_eq!(grid.next_row_segment(x, x_max, "1", "2", "3"), "3");
    }
}
