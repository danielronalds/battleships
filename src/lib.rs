mod grid;
pub mod point;

pub const BOARD_SIZE: u8 = 10;

use std::io;

use grid::Grid;

use point::Point;

pub struct BattleshipGame {
    grid: Grid,
    battleships: Vec<Point>,
    battleships_to_hit: usize,
    hits: Vec<Point>,
    misses: Vec<Point>,
}

impl BattleshipGame {
    /// Creates a new BattleshipGame instance
    pub fn new(battleships: Vec<Point>) -> Self {
        let battleships_to_hit = battleships.len();
        Self {
            grid: Grid::new(BOARD_SIZE, BOARD_SIZE),
            battleships,
            battleships_to_hit,
            hits: Vec::new(),
            misses: Vec::new(),
        }
    }

    pub fn play(&mut self) {
        loop {
            println!("{}", self.grid.write_grid(&self.hits, &self.misses));

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

            if self.battleships.contains(&guess) {
                // Unwrap is safe here as we have confirmed that postion will return something as
                // this code only runs if the guess is in battleships
                let hit_index = self.battleships
                    .iter()
                    .position(|point| point == &guess)
                    .unwrap();
                self.hits.push(guess);
                self.battleships.remove(hit_index);
            } else if !self.hits.contains(&guess) {
                self.misses.push(guess);
            }

            if self.hits.len() == self.battleships_to_hit {
                break;
            }
        }

        println!("{}\nYou won!", self.grid.write_grid(&self.hits, &self.misses));
    }
}

#[cfg(test)]
mod tests {}
