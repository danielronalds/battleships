mod grid;
mod point;
mod ship_placer;

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
    pub fn new() -> Self {
        let grid = Grid::new(BOARD_SIZE, BOARD_SIZE);
        let battleships = ship_placer::place_ships(&grid);
        let battleships_to_hit = battleships.len();
        Self {
            grid,
            battleships,
            battleships_to_hit,
            hits: Vec::new(),
            misses: Vec::new(),
            lines_to_jump: 0,
        }
    }

    /// Plays a game of battleship
    pub fn play(&mut self, print_ships: bool) {
        if print_ships {
            println!("{}", self.grid.write_grid(&self.battleships, &self.misses));
        }

        while !self.game_over() {
            self.reset_screen();

            println!("{}", self.grid.write_grid(&self.hits, &self.misses));
            self.lines_to_jump += 23;

            println!("Guess in format (Column Letter)(Row Number) for example: A2");
            let mut guess = String::new();
            stdin().read_line(&mut guess).expect("Couldn't read line");
            self.lines_to_jump += 2;

            let mut guess = match Point::try_from(guess) {
                Ok(guess) => guess,
                Err(message) => {
                    eprintln!("{}", message);
                    stdin().read_line(&mut String::new()).unwrap();
                    self.lines_to_jump += 2;
                    continue;
                }
            };

            guess.invert();

            self.process_guess(guess);
        }

        self.reset_screen();

        println!(
            "{}\nYou won!",
            self.grid.write_grid(&self.hits, &self.misses)
        );
    }

    /// Checks if the game is over, based on whether the number of hits is equal to the number of
    /// battleships to hit
    fn game_over(&self) -> bool {
        self.hits.len() == self.battleships_to_hit
    }

    /// Figures out whether the guess was a hit or a miss, and then adds it to the appropriate
    /// vector
    ///
    /// Parameters:
    /// guess:   The players guess as a point
    fn process_guess(&mut self, guess: Point) {
        if self.battleships.contains(&guess) {
            // Unwrap is safe here as we have confirmed that position will return something as
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

impl Default for BattleshipGame {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {}
