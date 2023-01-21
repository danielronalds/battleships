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

        let _end_of_column = self.height - 1;
        let end_of_row = self.width.saturating_sub(1);

        for y in 0..self.width {
            for x in 0..self.width {
                if x == 0 && y == 0 {
                    grid.push_str("╭───┬");
                } else if x == end_of_row && y == 0 {
                    grid.push_str("───╮");
                } else if y == 0 {
                    grid.push_str("───┬");
                }
            }
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
