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
    pub fn write_grid(&self) -> String {
        let mut grid = String::new();

        let end_of_column = self.height.saturating_sub(1);
        let end_of_row = self.width.saturating_sub(1);

        for y in 0..self.height {
            if y == 0 {
                for x in 0..self.width {
                    grid.push_str(self.next_row_segment(x, end_of_row, "╭───┬", "───┬", "───╮"));
                }
                grid.push_str("\n");
            }

            for x in 0..self.width {
                grid.push_str(self.next_row_segment(x, end_of_row, "│   │", "   │", "   │"));
            }

            grid.push_str("\n");

            for x in 0..self.width {
                if y == end_of_column {
                    grid.push_str(self.next_row_segment(x, end_of_row, "╰───┴", "───┴", "───╯"));
                } else {
                    grid.push_str(self.next_row_segment(x, end_of_row, "├───┼", "───┼", "───┤"));
                }
            }

            grid.push_str("\n");
        }

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
    fn write_grid_with_one_row_works() {
        // Testing with 4
        let grid = Grid::new(4, 1);

        let expected_result = "╭───┬───┬───┬───╮\n│   │   │   │   │\n╰───┴───┴───┴───╯\n".to_owned();

        assert_eq!(expected_result, grid.write_grid());

        // testing with 3
        let grid = Grid::new(3, 1);

        let expected_result = "╭───┬───┬───╮\n│   │   │   │\n╰───┴───┴───╯\n".to_owned();

        assert_eq!(expected_result, grid.write_grid());
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
