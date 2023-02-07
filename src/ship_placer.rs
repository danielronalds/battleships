use crate::grid::Grid;
use crate::point::Point;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use ShipDirection::*;

/// The length of the ships to place. In reverse order as it will be more difficult to place the
/// larger ships last
const SHIPS_TO_PLACE: [i32; 5] = [5, 4, 3, 3, 2];

enum ShipDirection {
    Horizontal,
    Vertical,
}

// Implementing this trait as it allows rand::random() to be used on the ShipDirection enum
impl Distribution<ShipDirection> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ShipDirection {
        match rng.gen_range(0..=1) {
            0 => ShipDirection::Horizontal,
            _ => ShipDirection::Vertical,
        }
    }
}

pub fn place_ships(grid: &Grid) -> Vec<Point> {
    let mut battleships: Vec<Point> = Vec::new();

    for ship_length in SHIPS_TO_PLACE {
        let mut valid_ship = false;

        while !valid_ship {
            let mut ship_point = new_random_point(grid.width(), grid.height());
            let random_direction: ShipDirection = rand::random();

            let mut ship: Vec<Point> = vec![ship_point.clone()];

            for _i in 0..ship_length {
                match random_direction {
                    Horizontal => {
                        ship_point.x += 1;
                    }
                    Vertical => {
                        ship_point.y += 1;
                    }
                }
                ship.push(ship_point.clone());
            }

            if overlapping_elements(&battleships, &ship) {
                // If the battleship contains even a single point then there is an overlap, and the
                // ship is not valid... aka try again
                continue;
            }

            let last_point = ship.last().unwrap();

            if  last_point.x >= grid.width() || last_point.y >= grid.height() {
                continue;
            }

            battleships.append(&mut ship.clone());
            valid_ship = true;
        }

    }

    battleships
}

fn overlapping_elements(vec: &Vec<Point>, other_vec: &Vec<Point>) -> bool {
    for element in vec {
        if other_vec.contains(element) {
            return true;
        }
    }

    false
}

/// Returns a new random_point within the given width and height
///
/// Parameters
/// width:    The width bound
/// height:   The height bound
fn new_random_point(width: u8, height: u8) -> Point {
    let random_point_x = rand::thread_rng().gen_range(0..width);
    let random_point_y = rand::thread_rng().gen_range(0..height);
    Point::new(random_point_x, random_point_y)
}
