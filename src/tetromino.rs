use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::board::{
    init_board,
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
    pub fn get_structure(&self) -> [[CellState; 4]; 4] {
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

#[derive(Resource)]
struct DropTimer {
    timer: Timer,
}

#[derive(Component)]
pub struct Tetromino {
    pub shape: Shape,
    pub queue: Vec<Shape>,
    pub progress: usize,
    pub shift: usize,
}

pub struct TetromiroPlugin;
impl Plugin for TetromiroPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(spawn_tetro.after(init_board))
            .add_system(drop_tetro);
    }
}

fn random_tetro() -> Shape {
    use Shape::*;
    let variants = [Straight, Square, T, L, Skew, LMir, SkewMir];
    let i = rand::thread_rng().gen_range(0..variants.len());
    variants[i].to_owned()
}

fn drop_tetro(mut tetro: Query<&mut Tetromino>, time: Res<Time>, mut drop_timer: ResMut<DropTimer>) {
    drop_timer.timer.tick(time.delta());

    if drop_timer.timer.finished() {
        let mut tetro = tetro.single_mut();
        tetro.progress += 1;
    }
}

fn spawn_tetro(mut commands: Commands) {
    commands.insert_resource(DropTimer {
        timer: Timer::new(Duration::from_millis(500), TimerMode::Repeating),
    });

    commands.spawn(Tetromino {
        shape: random_tetro(),
        queue: vec![],
        progress: 0,
        shift: 3,
    });
}
