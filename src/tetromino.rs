use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::board::{
    init_board, Board,
    CellState::{self, *},
};

use Collision::*;
use Shape::*;

const ALL_SHAPE: [Shape; 7] = [Straight, Square, T, L, Skew, LMir, SkewMir];

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
    pub fn get_structure(&self) -> [[CellState; 4]; 4] {
        match self {
            Straight => [[E; 4], [E; 4], [E; 4], [F; 4]],
            Square => [[E; 4], [E; 4], [E, F, F, E], [E, F, F, E]],
            T => [[E; 4], [E; 4], [E, F, E, E], [F, F, F, E]],
            L => [[E; 4], [E, F, F, E], [E, F, E, E], [E, F, E, E]],
            Skew => [[E; 4], [E; 4], [F, F, E, E], [E, F, F, E]],
            LMir => [[E; 4], [E, F, F, E], [E, E, F, E], [E, E, F, E]],
            SkewMir => [[E; 4], [E; 4], [E, F, F, E], [F, F, E, E]],
        }
    }
}

#[derive(Resource)]
struct DropTimer {
    timer: Timer,
}

#[derive(Component, Clone)]
pub struct Tetromino {
    pub structure: [[CellState; 4]; 4],
    pub queue: Vec<Shape>,
    pub progress: usize,
    pub shift: usize,
}

impl Tetromino {
    fn next_tetro(&mut self) {
        if self.queue.len() == 0 {
            let mut buf: Vec<Shape> = ALL_SHAPE.into_iter().collect();
            for _ in 0..buf.len() {
                let i = rand::thread_rng().gen_range(0..buf.len());
                self.queue.push(buf.remove(i));
            }
        }
        self.structure = self.queue.pop().unwrap().get_structure();
    }

    fn new() -> Self {
        let mut q: Vec<Shape> = Vec::new();
        let mut buf: Vec<Shape> = ALL_SHAPE.into_iter().collect();
        for _ in 0..buf.len() {
            let i = rand::thread_rng().gen_range(0..buf.len());
            q.push(buf.remove(i));
        }

        let s = q.pop().unwrap().get_structure();

        Tetromino {
            structure: s,
            queue: q,
            progress: 0,
            shift: 3,
        }
    }
}

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
enum Collision {
    Side,
    Bottom,
    Block,
}

pub struct TetromiroPlugin;
impl Plugin for TetromiroPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_tetro.after(init_board))
            .add_system(drop_tetro);
    }
}

fn detect_collision(board: &Board, tetro: &Tetromino) -> Option<Collision> {
    use Collision::*;
    for (i_row, row) in tetro.structure.iter().enumerate() {
        for (i_col, col) in row.iter().enumerate() {
            if *col == E {
                continue;
            }

            if (i_row as isize + 16 - tetro.progress as isize) < 0 {
                return Some(Bottom);
            }

            if board.placed[i_row + 16 - tetro.progress][i_col + tetro.shift] == F {
                return Some(Block);
            }
        }
    }
    None
}

fn drop_tetro(
    mut tetro: Query<&mut Tetromino>,
    mut board: Query<&mut Board>,
    time: Res<Time>,
    mut drop_timer: ResMut<DropTimer>,
) {
    drop_timer.timer.tick(time.delta());
    if !drop_timer.timer.finished() {
        return;
    }

    let mut tetro = tetro.single_mut();
    let mut board = board.single_mut();
    tetro.progress += 1;
    match detect_collision(&board, &tetro) {
        None => (),
        Some(Bottom) | Some(Block) => {
            tetro.progress -= 1;
            for (i_row, row) in tetro.structure.iter().enumerate() {
                for (i_col, col) in row.iter().enumerate() {
                    if *col != F {
                        continue;
                    }
                    board.placed[i_row + 16 - tetro.progress][i_col + tetro.shift] = F;
                }
            }
            tetro.progress = 0;
            tetro.next_tetro();
        }
        Some(Side) => (),
    }
}

fn spawn_tetro(mut commands: Commands) {
    commands.insert_resource(DropTimer {
        timer: Timer::new(Duration::from_millis(300), TimerMode::Repeating),
    });

    commands.spawn(Tetromino::new());
}

#[cfg(test)]
mod test {
    use crate::{
        board::{Board, CellState::*},
        tetromino::{detect_collision, Collision::*},
    };

    use super::{Shape, Tetromino};

    #[test]
    fn detect_collision_test() {
        let mut board = Board::default();
        board.placed = [[E; 10]; 20];
        board.placed[18] = [F; 10];
        let tetro = Tetromino {
            structure: Shape::Square.get_structure(),
            queue: Vec::new(),
            progress: 1,
            shift: 3,
        };
        let tetro2 = Tetromino {
            structure: Shape::Straight.get_structure(),
            queue: Vec::new(),
            progress: 1,
            shift: 3,
        };
        let tetro3 = Tetromino {
            structure: Shape::L.get_structure(),
            queue: Vec::new(),
            progress: 19,
            shift: 3,
        };

        assert_eq!(Some(Block), detect_collision(&board, &tetro));
        assert_eq!(Some(Block), detect_collision(&board, &tetro2));
        dbg!(detect_collision(&board, &tetro3));
    }
}
