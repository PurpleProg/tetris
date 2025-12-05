use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    widgets::{Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};

mod bag;
mod grid;
mod tetromino;
mod vec2;
use crate::bag::*;
use crate::grid::*;

const SOFT_DROP_SPEED: f32 = 0.05;
const NORMAL_SPEED: f32 = 0.5;

const CREDITS: &str = "Tetris
Author : mphippen
Source : https://github.com/PurpleProg/tetris

rust > C";

// let delay = Duration::from_secs_f32(NORMAL_SPEED);

#[derive(Debug)]
enum GameEvent {
    GameOver,
    Quit,
    Tick,
}

// TODO:
// remove expects (rendererrors)
// gameover -> replay ?
// speed
// score -> save -> multiplayer NOTE: very fun ! but easy to cheat
// wall-kicks
// bag preview -> next piece preview (anoying as fuck cause i have to pre-shot the next bag) or no ? if i refill when size is one

fn main() -> () {
    let mut terminal = ratatui::init();

    // setup game vars
    let mut delta_time: Duration;
    let mut previous_time = Instant::now();
    let mut time_since_last_move = Duration::new(0, 0);
    let mut grid: Grid = [[None; GRID_WIDTH]; GRID_HEIGHT];
    let delay = Duration::from_secs_f32(NORMAL_SPEED);
    let mut bag = new_bag();

    'gameloop: loop {
        delta_time = Instant::now() - previous_time;
        time_since_last_move += delta_time;
        previous_time = Instant::now();

        match update(&mut bag, &mut grid, &mut time_since_last_move, delay) {
            GameEvent::GameOver => {
                ratatui::restore();
                println!("Game Over :(");
                return;
            }
            GameEvent::Tick => {}
            GameEvent::Quit => break 'gameloop,
        }
        render(&bag, grid, &mut terminal);
    }
    ratatui::restore();
}

fn update(
    bag: &mut Bag,
    grid: &mut Grid,
    time_since_last_move: &mut Duration,
    mut delay: Duration,
) -> GameEvent {
    let mut next_tetromino = bag.last().expect("bag empty at start of update :/").clone();

    if event::poll(Duration::from_secs(0)).unwrap_or(false) {
        if let Ok(Event::Key(key)) = event::read() {
            match key.code {
                KeyCode::Esc => return GameEvent::Quit,
                KeyCode::Char('k') => next_tetromino.rotate(),
                KeyCode::Up => next_tetromino.rotate(),
                KeyCode::Char('j') => next_tetromino.rotate(), // NOTE: maybe make a ccw vesion of rotate
                KeyCode::Char('l') => next_tetromino.pos.x += 1,
                KeyCode::Right => next_tetromino.pos.x += 1,
                KeyCode::Char('h') => next_tetromino.pos.x -= 1,
                KeyCode::Left => next_tetromino.pos.x -= 1,
                KeyCode::Char(' ') => delay = Duration::from_secs_f32(SOFT_DROP_SPEED),
                _ => {}
            }
        }
    }

    // sideways collisions
    if next_tetromino.collide(grid).is_some() {
        // TODO: wall kicks
        // NOTE: this skip the gravity check, allowing for holding the piece against a wall
        // NOTE: it dont ? im leaving it, if you manage to use that bug go on
        return GameEvent::Tick;
    }

    // move down
    if *time_since_last_move > delay {
        next_tetromino.pos.y += 1;
        *time_since_last_move = Duration::ZERO;
    }

    // ground collision
    if next_tetromino.collide(grid).is_some() {
        // place tetromino on grid
        bag.pop()
            .expect("bag empty on groud col")
            .stamp_onto(grid)
            .expect("tetromino move de-sync");
        // refill bag
        if bag.is_empty() {
            *bag = new_bag();
        }
        // check if the next tetromino will cause a game over
        if bag.last().expect("bag empty").collide(&grid).is_some() {
            return GameEvent::GameOver;
        }
        return GameEvent::Tick;
    }

    // move
    *bag.last_mut().expect("bag empty on move") = next_tetromino.clone();
    clear_lines(grid);
    GameEvent::Tick
}
fn render(bag: &Bag, grid: Grid, terminal: &mut DefaultTerminal) -> () {
    let area = terminal.get_frame().area();
    let cell_height = area.height / GRID_HEIGHT as u16;
    let cell_width = cell_height * 2;

    let vertical_rect = Rect {
        x: 0,
        y: 0,
        width: area.width,
        // + 2 offset to avoid overlapping the borders (each sides)
        height: cell_height * GRID_HEIGHT as u16 + 2,
    };

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            // + 2 offset to avoid overlapping the borders (each sides)
            Constraint::Length(cell_width * GRID_WIDTH as u16 + 2),
            Constraint::Fill(1),
        ])
        .split(vertical_rect);

    let left_panel = Paragraph::new(CREDITS)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_set(symbols::border::ROUNDED)
                .border_style(Style::default().fg(ratatui::style::Color::DarkGray))
                .title(" Tetris ")
                .title_alignment(Alignment::Center)
                .title_style(Style::default().fg(Color::White)),
        );

    let right_panel = Block::default()
        .borders(Borders::ALL)
        .border_set(symbols::border::ROUNDED)
        .border_style(Style::default().fg(ratatui::style::Color::DarkGray))
        .title(" Leaderboard - coming soon ")
        .title_alignment(Alignment::Center)
        .title_style(Style::default().fg(Color::White));

    let playfield = Block::default()
        .borders(Borders::ALL)
        .border_set(symbols::border::ROUNDED)
        .border_style(Style::default().fg(ratatui::style::Color::DarkGray))
        .title(" Playfield ")
        .title_alignment(Alignment::Center)
        .title_style(Style::default().fg(Color::White));

    // create a new temp grid that hold the current tetromino
    let mut grid_with_tetromino = grid.clone();
    bag.last()
        .expect("bag empty in rendering")
        .stamp_onto(&mut grid_with_tetromino)
        .expect("collision cauth in render, sould've been cauth in update");

    terminal
        .draw(|frame| {
            frame.render_widget(left_panel, layout[0]);
            frame.render_widget(playfield, layout[1]);
            frame.render_widget(right_panel, layout[2]);

            for (i, line) in grid_with_tetromino.iter().enumerate() {
                for (j, cell) in line.iter().enumerate() {
                    // + 1 offset to avoid overlapping the border
                    let y = layout[1].y + 1 + (i as u16) * cell_height;
                    let x = layout[1].x + 1 + (j as u16) * cell_width;

                    let cell_rect = Rect {
                        x: x,
                        y: y,
                        width: cell_width,
                        height: cell_height,
                    };

                    let block = Block::default()
                        .borders(Borders::NONE)
                        .style(if let Some(color) = cell {
                            Style::default().fg(*color).bg(*color)
                        } else {
                            Style::default()
                        })
                        .title(if cell.is_some() { "" } else { "." }); // '█'
                    frame.render_widget(block, cell_rect);
                }
            }
        })
        .expect("ratatui rendering error");
}
