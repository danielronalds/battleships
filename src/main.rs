use battleships::{point::Point, BattleshipGame, grid::Grid, ship_placer};

fn main() {
    let battleships = vec![
        Point::new(0, 0),
        Point::new(0, 1),
        Point::new(0, 2),
        Point::new(0, 3),
        Point::new(0, 4),
    ];

    // let mut battleships_game = BattleshipGame::new(battleships);

    // battleships_game.play();

    let grid = Grid::new(battleships::BOARD_SIZE, battleships::BOARD_SIZE);

    let battleships = ship_placer::place_ships(&grid);

    println!("{}", grid.write_grid(&battleships, &vec![]));
}
