mod grid;
mod point;

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

        io::stdin()
            .read_line(&mut guess)
            .expect("Couldn't read line");

        let x = letter_to_coordinate(guess.chars().nth(0).unwrap()).unwrap();
        let y: u8 = guess[1..2].parse().unwrap();
        let y: u8 = y.saturating_sub(1);

        println!("{x}, {y}");
    }
}

/// Converts the given coordinate to a coordinate. This function does convert the letter to it's
/// upper varient. Returns None if the letter is nonalphabetic or fails to convert to a captital
///
/// Parameters
/// letter:   The letter to convert
fn letter_to_coordinate(letter: char) -> Option<u8> {
    if !letter.is_alphabetic() {
        return None;
    }

    let letter_upper = letter
        .to_uppercase()
        .collect::<Vec<char>>()
        .first()?
        .to_owned();

    let letter_as_u8 = letter_upper as u8;
    let letter_as_coordinate = letter_as_u8 - 65; // Taking away 65 as A in ascii is 65

    Some(letter_as_coordinate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letter_to_coordinate_works() {
        assert_eq!(letter_to_coordinate('A'), Some(0));
        assert_eq!(letter_to_coordinate('C'), Some(2));
        assert_eq!(letter_to_coordinate('c'), Some(2));
        assert_eq!(letter_to_coordinate('z'), Some(25));
        assert_eq!(letter_to_coordinate('Z'), Some(25));

        assert_eq!(letter_to_coordinate('-'), None);
        assert_eq!(letter_to_coordinate('4'), None);
        assert_eq!(letter_to_coordinate('^'), None);
    }
}
