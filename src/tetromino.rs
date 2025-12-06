use ratatui::style::Color;

use crate::grid::*;
use crate::vec2::Vec2;

#[derive(Debug)]
pub enum Collision {
    Occupied,
    OutOfBound,
}

#[derive(Debug, Clone)]
pub struct Tetromino {
    pub blocks: Vec<Vec2>,
    pub does_rotate: bool,
    pub pos: Vec2,
    pub color: Color,
}
impl Default for Tetromino {
    fn default() -> Self {
        Tetromino {
            blocks: vec![
                Vec2::new(0, 0),
                Vec2::new(-1, -1),
                Vec2::new(-1, 1),
                Vec2::new(1, -1),
                Vec2::new(1, 1),
            ],
            does_rotate: true,
            pos: Vec2 {
                x: GRID_WIDTH as i8 / 2,
                y: 1,
            },
            color: Color::Red,
        }
    }
}
impl Tetromino {
    pub fn stamp_onto(&self, grid: &mut Grid) -> Result<(), Collision> {
        self.blocks.iter().try_for_each(|block| {
            let grid_x = self.pos.x + block.x;
            let grid_y = self.pos.y + block.y;
            if grid_y < 0
                || grid_x < 0
                || grid_x as usize >= GRID_WIDTH
                || grid_y as usize >= GRID_HEIGHT
            {
                return Err(Collision::OutOfBound);
            }
            if grid[grid_y as usize][grid_x as usize].is_some() {
                return Err(Collision::Occupied);
            }
            grid[grid_y as usize][grid_x as usize] = Some(self.color);
            Ok(())
        })
    }
    pub fn rotate(&mut self) -> () {
        if !self.does_rotate {
            return;
        }
        self.blocks.iter_mut().for_each(|block| {
            (block.x, block.y) = (-block.y, block.x);
        });
    }
    pub fn collide(&self, grid: &Grid) -> bool {
        for block in self.blocks.iter() {
            let x = self.pos.x + block.x;
            let y = self.pos.y + block.y;
            if x as usize >= GRID_WIDTH || y as usize >= GRID_HEIGHT || x < 0 || y < 0 {
                return true;
            }
            if grid[y as usize][x as usize].is_some() {
                return true;
            }
        }
        false
    }
    pub fn try_move_down(&mut self, grid: &Grid) -> Result<(), ()> {
        self.pos.y += 1;
        if self.collide(grid) {
            self.pos.y -= 1;
            Err(())
        } else {
            Ok(())
        }
    }
}
