pub mod point;
mod grid;

pub const BOARD_SIZE: u8 = 10;

use std::io;

use grid::Grid;

use point::Point;

/// Creates a new game of battleships
pub fn new_game(battleships: Vec<Point>) {
    let mut game_over = false;

    let grid = Grid::new(BOARD_SIZE, BOARD_SIZE);

    let mut hits = Vec::<Point>::new();
    let mut misses = Vec::<Point>::new();

    while !game_over {
        println!("{}", grid.write_grid(&hits, &misses));

        println!("Guess in format (Column Letter)(Row Number) for example: A2");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Couldn't read line");

        let guess = match Point::try_from(guess) {
            Ok(guess) => guess,
            Err(message) => {
                eprintln!("{}", message);
                continue;
            }
        };

        match battleships.contains(&guess) {
            true => hits.push(guess),
            false => misses.push(guess),
        }

        if hits.len() == battleships.len() {
            game_over = true;
        }
    }

    println!("{}\nYou won!", grid.write_grid(&hits, &misses));
}

#[cfg(test)]
mod tests {}
