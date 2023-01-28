use battleships::{ point::Point };

fn main() {
    let battleships = vec![
        Point::new(0, 0),
        Point::new(0, 1),
        Point::new(0, 2),
        Point::new(0, 3),
        Point::new(0, 4)
    ];

    battleships::new_game(battleships);
}
