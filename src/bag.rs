use crate::tetromino::Tetromino;
use crate::vec2::Vec2;
use rand::prelude::SliceRandom;
use rand::rng;
use ratatui::style::Color::*;

pub type Bag = Vec<Tetromino>;

pub fn new_bag() -> Bag {
    let mut bag: Vec<Tetromino> = vec![
        Tetromino {
            blocks: vec![
                Vec2 { x: -1, y: 0 },
                Vec2 { x: 0, y: 0 },
                Vec2 { x: 1, y: 0 },
                Vec2 { x: 2, y: 0 },
            ],
            color: Red,
            ..Default::default()
        },
        Tetromino {
            blocks: vec![
                Vec2 { x: 0, y: 0 },
                Vec2 { x: 1, y: 0 },
                Vec2 { x: 0, y: 1 },
                Vec2 { x: 1, y: 1 },
            ],
            color: Blue,
            does_rotate: false,
            ..Default::default()
        },
        Tetromino {
            blocks: vec![
                Vec2::new(0, 0),
                Vec2::new(-1, 0),
                Vec2::new(0, 1),
                Vec2::new(1, 1),
            ],
            color: Green,
            ..Default::default()
        },
        Tetromino {
            blocks: vec![
                Vec2::new(0, 0),
                Vec2::new(1, 0),
                Vec2::new(0, 1),
                Vec2::new(-1, 1),
            ],
            color: Cyan,
            ..Default::default()
        },
        Tetromino {
            blocks: vec![
                Vec2::new(0, 0),
                Vec2::new(1, 0),
                Vec2::new(-1, 0),
                Vec2::new(0, 1),
            ],
            color: Yellow,
            ..Default::default()
        },
        Tetromino {
            blocks: vec![
                Vec2::new(-1, 1),
                Vec2::new(-1, 0),
                Vec2::new(0, 0),
                Vec2::new(1, 0),
            ],
            color: Magenta,
            ..Default::default()
        },
        Tetromino {
            blocks: vec![
                Vec2::new(1, 1),
                Vec2::new(1, 0),
                Vec2::new(0, 0),
                Vec2::new(-1, 0),
            ],
            color: White,
            ..Default::default()
        },
    ];
    bag.shuffle(&mut rng());
    bag
}
