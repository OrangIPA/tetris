use bevy::prelude::*;
use CellState::*;

use crate::tetromino::Tetromino;

pub const CELL_SIZE: f32 = 30.;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum CellState {
    #[default]
    E,
    F,
}

#[derive(Component)]
struct Cell {
    x: isize,
    y: isize,
}

#[derive(Component, Default)]
pub struct Board {
    pub state: [[CellState; 10]; 20],
    pub placed: [[CellState; 10]; 20],
}

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_board).add_system(update_cells)
            .add_system(show_cells);
    }
}

pub fn init_board(mut commands: Commands) {
    for i in 0..20 {
        for j in 0..10 {
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: if (i + j) % 2 == 0 {
                            Color::rgba(0.45, 0.45, 0.45, 0.3)
                        } else {
                            Color::rgba(0.45, 0.45, 0.45, 0.3)
                        },
                        custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        (j as f32 * CELL_SIZE) - (CELL_SIZE * 5.),
                        (i as f32 * CELL_SIZE) - (CELL_SIZE * 10.),
                        0.,
                    )),
                    ..Default::default()
                })
                .insert(Cell { x: j, y: i });
        }
    }

    commands.spawn(Board::default());
}

fn update_cells(mut cells: Query<(&mut Sprite, &Cell)>, board: Query<&Board>) {
    let board = board.single();
    for cell in cells.iter_mut() {
        let (mut cell, pos) = cell;
        let cell_state = board.state[pos.y as usize][pos.x as usize];
        match cell_state {
            E => cell.color = Color::rgb(0.45, 0.45, 0.45),
            F => cell.color = Color::rgb(1., 1., 1.),
        }
    }
}

fn show_cells(mut board: Query<&mut Board>, tetro: Query<&Tetromino>) {
    let tetro = match tetro.get_single() {
        Ok(v) => v,
        Err(_) => return,
    };
    let mut board = match board.get_single_mut() {
        Ok(v) => v,
        Err(_) => return,
    };

    for (i_row, row) in board.placed.clone().iter().enumerate() {
        for (i_col, col) in row.iter().enumerate() {
            board.state[i_row][i_col] = *col;
        }
    }

    for (i_row, row) in tetro.structure.iter().enumerate() {
        for (i_col, col) in row.iter().enumerate() {
            if 16 + i_row < tetro.progress { continue; }
            if *col != F { continue; }
            board.state[16 + i_row - tetro.progress][i_col + tetro.shift] = F;
        }
    }
}
