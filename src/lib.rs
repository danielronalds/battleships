mod grid;
pub mod point;

pub const BOARD_SIZE: u8 = 10;

use std::io::{stdin, stdout};

use grid::Grid;

use point::Point;

use crossterm::{cursor, execute, terminal};

pub struct BattleshipGame {
    grid: Grid,
    battleships: Vec<Point>,
    battleships_to_hit: usize,
    hits: Vec<Point>,
    misses: Vec<Point>,
    lines_to_jump: u16,
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
            lines_to_jump: 0,
        }
    }

    pub fn play(&mut self) {
        loop {
            self.reset_screen();

            println!("{}", self.grid.write_grid(&self.hits, &self.misses));
            self.lines_to_jump += 23;

            println!("Guess in format (Column Letter)(Row Number) for example: A2");
            let mut guess = String::new();
            stdin().read_line(&mut guess).expect("Couldn't read line");
            self.lines_to_jump += 2;

            let guess = match Point::try_from(guess) {
                Ok(guess) => guess,
                Err(message) => {
                    eprintln!("{}", message);
                    self.lines_to_jump += 1;
                    continue;
                }
            };

            if self.battleships.contains(&guess) {
                // Unwrap is safe here as we have confirmed that postion will return something as
                // this code only runs if the guess is in battleships
                let hit_index = self
                    .battleships
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

        println!(
            "{}\nYou won!",
            self.grid.write_grid(&self.hits, &self.misses)
        );
    }

    /// Clears the screen from the cursor down
    fn reset_screen(&mut self) {
        execute!(
            stdout(),
            cursor::MoveUp(self.lines_to_jump),
            terminal::Clear(terminal::ClearType::FromCursorDown)
        )
        .unwrap();

        self.lines_to_jump = 0;
    }
}

#[cfg(test)]
mod tests {}
