use battleships::BattleshipGame;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
/// Play battleships in your terminal!
pub struct Args {
    #[arg(long)]
    /// Whether to show the location of the enemy ships or not
    show_ships: bool,
}

fn main() {
    let mut battleships_game = BattleshipGame::new();

    let show_ships = Args::parse().show_ships;

    battleships_game.play(show_ships);
}
