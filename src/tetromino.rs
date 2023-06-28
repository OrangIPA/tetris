use bevy::prelude::*;
use rand::Rng;

use crate::board::{
    init_board, Board,
    CellState::{self, *},
};

#[derive(Clone)]
pub enum Shape {
    Straight,
    Square,
    T,
    L,
    Skew,
    LMir,
    SkewMir,
}

impl Shape {
    fn get_structure(&self) -> [[CellState; 4]; 4] {
        match self {
            Shape::Straight => [[E; 4], [E; 4], [E; 4], [F; 4]],
            Shape::Square => [[E; 4], [E; 4], [E, F, F, E], [E, F, F, E]],
            Shape::T => [[E; 4], [E; 4], [E, F, E, E], [F, F, F, E]],
            Shape::L => [[E; 4], [E, F, F, E], [E, F, E, E], [E, F, E, E]],
            Shape::Skew => [[E; 4], [E; 4], [F, F, E, E], [E, F, F, E]],
            Shape::LMir => [[E; 4], [E, F, F, E], [E, E, F, E], [E, E, F, E]],
            Shape::SkewMir => [[E; 4], [E; 4], [E, F, F, E], [F, F, E, E]],
        }
    }
}

#[derive(Component)]
pub struct Tetromino {
    shape: Shape,
    queue: Vec<Shape>,
    proggress: usize,
}

pub struct TetromiroPlugin;
impl Plugin for TetromiroPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_tetro.after(init_board))
            .add_system(show_tetro);
    }
}

fn random_tetro() -> Shape {
    use Shape::*;
    let variants = [Straight, Square, T, L, Skew, LMir, SkewMir];
    let i = rand::thread_rng().gen_range(0..variants.len());
    variants[i].to_owned()
}

fn spawn_tetro(mut commands: Commands) {
    commands.spawn(Tetromino {
        shape: random_tetro(),
        queue: vec![],
        proggress: 0,
    });
}

fn show_tetro(mut board: Query<&mut Board>, tetro: Query<&Tetromino>) {
    let tetro = match tetro.get_single() {
        Ok(v) => v,
        Err(_) => return,
    };
    let mut board = match board.get_single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    for (i_row, row) in tetro.shape.get_structure().iter().enumerate() {
        for (i_col, col) in row.iter().enumerate() {
            board.state[16 + i_row - tetro.proggress][i_col + 3] = *col;
        }
    }
}
