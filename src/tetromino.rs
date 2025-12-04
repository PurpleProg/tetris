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
    pub pos: Vec2,
    pub color: Color,
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
        self.blocks.iter_mut().for_each(|block| {
            (block.x, block.y) = (-block.y, block.x);
        });
    }
    pub fn collide(&self, grid: &Grid) -> Option<Collision> {
        for block in self.blocks.iter() {
            let x = self.pos.x + block.x;
            let y = self.pos.y + block.y;
            if x as usize >= GRID_WIDTH || y as usize >= GRID_HEIGHT || x < 0 || y < 0 {
                return Some(Collision::OutOfBound);
            }
            if grid[y as usize][x as usize].is_some() {
                return Some(Collision::Occupied);
            }
        }
        None
    }
}
