// Grid for representing a
pub struct Grid {
    width: u8,
    height: u8,
}

impl Grid {
    /// Constructs a new Grid struct with the given width and height
    ///
    /// Paremeters
    /// width:    The width of the grid
    /// height:   The height of the grid
    pub fn new(width: u8, height: u8) -> Self {
        Self { width, height }
    }

    pub fn write_grid(&self) -> String {
        let mut grid = String::new();

        let end_of_column = self.height.saturating_sub(1);
        let end_of_row = self.width.saturating_sub(1);

        for y in 0..self.height {
            if y == 0 {
                for x in 0..self.width {
                    if x == 0 {
                        grid.push_str("╭───┬");
                    } else if x == end_of_row {
                        grid.push_str("───╮");
                    } else {
                        grid.push_str("───┬");
                    }
                }
                grid.push_str("\n");
            }

            for x in 0..self.width {
                if x == 0 {
                    grid.push_str("│   │");
                } else if x == end_of_row {
                    grid.push_str("   │");
                } else {
                    grid.push_str("   │");
                }
            }

            grid.push_str("\n");

            for x in 0..self.width {
                if y == end_of_column {
                    if x == 0 {
                        grid.push_str("╰───┴");
                    } else if x == end_of_row {
                        grid.push_str("───╯");
                    } else {
                        grid.push_str("───┴");
                    }
                } else {
                    if x == 0 {
                        grid.push_str("├───┼");
                    } else if x == end_of_row {
                        grid.push_str("───┤");
                    } else {
                        grid.push_str("───┼");
                    }
                }
            }

            grid.push_str("\n");
        }

        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Checks if the constructor creates a Grid stuct with the right width and height
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

        let expected_result = "╭───┬───┬───┬───╮".to_owned();

        assert_eq!(expected_result, grid.write_grid());

        // testing with 3
        let grid = Grid::new(3, 1);

        let expected_result = "╭───┬───┬───╮".to_owned();

        assert_eq!(expected_result, grid.write_grid());
    }
}
