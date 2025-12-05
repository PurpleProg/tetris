use crate::grid::*;
use crate::tetromino::Tetromino;
use crate::vec2::Vec2;
use rand::prelude::SliceRandom;
use rand::rng;
use ratatui::style::Color::*;

pub type Bag = Vec<Tetromino>;

const START_POS: Vec2 = Vec2 {
    x: GRID_WIDTH as i8 / 2,
    y: 1,
};

pub fn new_bag() -> Bag {
    let mut bag: Vec<Tetromino> = vec![
        Tetromino {
            blocks: vec![
                Vec2 { x: -1, y: 0 },
                Vec2 { x: 0, y: 0 },
                Vec2 { x: 1, y: 0 },
                Vec2 { x: 2, y: 0 },
            ],
            pos: START_POS,
            color: Red,
        },
        Tetromino {
            // wrong rotation center
            blocks: vec![
                Vec2 { x: 0, y: 0 },
                Vec2 { x: 1, y: 0 },
                Vec2 { x: 0, y: 1 },
                Vec2 { x: 1, y: 1 },
            ],
            pos: START_POS,
            color: Blue,
        },
        Tetromino {
            blocks: vec![
                Vec2::new(0, 0),
                Vec2::new(-1, 0),
                Vec2::new(0, 1),
                Vec2::new(1, 1),
            ],
            pos: START_POS,
            color: Green,
        },
        Tetromino {
            blocks: vec![
                Vec2::new(0, 0),
                Vec2::new(1, 0),
                Vec2::new(0, 1),
                Vec2::new(-1, 1),
            ],
            pos: START_POS,

            color: Cyan,
        },
        Tetromino {
            blocks: vec![
                Vec2::new(0, 0),
                Vec2::new(1, 0),
                Vec2::new(-1, 0),
                Vec2::new(0, 1),
            ],
            pos: START_POS,
            color: Yellow,
        },
        Tetromino {
            blocks: vec![
                Vec2::new(-1, 1),
                Vec2::new(-1, 0),
                Vec2::new(0, 0),
                Vec2::new(1, 0),
            ],
            pos: START_POS,
            color: Magenta,
        },
        Tetromino {
            blocks: vec![
                Vec2::new(1, 1),
                Vec2::new(1, 0),
                Vec2::new(0, 0),
                Vec2::new(-1, 0),
            ],
            pos: START_POS,
            color: White,
        },
    ];
    bag.shuffle(&mut rng());
    bag
}
