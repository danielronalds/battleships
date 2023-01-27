use battleships::{grid::Grid, point::Point };

fn main() {
    let _points = vec![
        Point::new(0, 1),
        Point::new(0, 2),
        Point::new(0, 3),
        Point::new(0, 4),

        Point::new(5, 3),
        Point::new(4, 3),
        Point::new(3, 3),
    ];

    battleships::new_game();
}
