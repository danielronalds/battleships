pub mod grid;
pub mod point;

pub const BOARD_SIZE: u8 = 10;

use std::io;

use grid::Grid;

use point::Point;

/// Creates a new game of battleships
pub fn new_game() {
    let game_over = false;

    let grid = Grid::new(BOARD_SIZE, BOARD_SIZE);

    let hits = Vec::<Point>::new();
    let misses = Vec::<Point>::new();

    while !game_over {
        println!("{}", grid.write_grid(&hits, &misses));

        println!("Guess in format [Column Letter][Row Number] for example: A2");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Couldn't read line");

        println!("{guess}");
    }
}

/// Converts the given coordinate to a coordinate. This function does convert the letter to it's
/// upper varient
///
/// Parameters
/// letter:   The letter to convert
fn letter_to_coordinate(letter: char) -> u8 {
    let letter_upper = letter.to_uppercase().collect::<Vec<char>>().first().unwrap().to_owned();

    let letter_as_u8 = letter_upper as u8;

    letter_as_u8 - 65
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letter_to_coordinate_works() {
        assert_eq!(letter_to_coordinate('A'), 0);
        assert_eq!(letter_to_coordinate('C'), 2);
        assert_eq!(letter_to_coordinate('c'), 2);
    }
}
