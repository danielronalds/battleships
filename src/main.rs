mod grid;
mod point;

use grid::Grid;
use point::Point;

fn main() {
    let grid = Grid::new(4, 4);

    let points = vec![
        Point::new(0, 1),
        Point::new(0, 2),
        Point::new(0, 3),
        Point::new(5, 3),
        Point::new(4, 3),
        Point::new(3, 3),
    ];

    println!("{}", grid.write_grid(points));
}
