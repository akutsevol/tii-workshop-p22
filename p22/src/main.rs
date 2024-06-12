mod tictac;

use crate::tictac::{Player, TicTacField};

fn main() {
    let mut field = TicTacField::new();
    field.make_move(0, 0, Player::X).unwrap();
    field.make_move(0, 1, Player::Y).unwrap();
    field.make_move(1, 1, Player::X).unwrap();
    field.make_move(0, 2, Player::Y).unwrap();
    field.make_move(2, 2, Player::X).unwrap();
    field.make_move(2, 0, Player::Y).unwrap();
    field.make_move(1, 0, Player::X).unwrap();
    field.make_move(1, 2, Player::Y).unwrap();
    field.make_move(2, 1, Player::X).unwrap();
    println!("{}", field.analyze().unwrap());
}
