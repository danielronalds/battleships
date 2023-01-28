mod grid;
pub mod point;

pub const BOARD_SIZE: u8 = 10;

use std::io;

use grid::Grid;

use point::Point;

/// Creates a new game of battleships
pub fn new_game(mut battleships: Vec<Point>) {
    let battleships_to_hit = battleships.len();

    let grid = Grid::new(BOARD_SIZE, BOARD_SIZE);

    let mut hits = Vec::<Point>::new();
    let mut misses = Vec::<Point>::new();

    loop {
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

        if battleships.contains(&guess) {
            // Unwrap is safe here as we have confirmed that postion will return something as
            // this code only runs if the guess is in battleships
            let hit_index = battleships
                .iter()
                .position(|point| point == &guess)
                .unwrap();
            hits.push(guess);
            battleships.remove(hit_index);
        } else if !hits.contains(&guess) {
            misses.push(guess);
        }

        if hits.len() == battleships_to_hit {
            break;
        }
    }

    println!("{}\nYou won!", grid.write_grid(&hits, &misses));
}

#[cfg(test)]
mod tests {}
