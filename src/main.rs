mod grid;

use grid::Grid;

fn main() {
    println!("Hello, world!");

    let grid = Grid::new(5, 5);

    println!("{}", grid.write_grid());
}
