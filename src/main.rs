use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    widgets::{Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tetromino::Tetromino;

mod bag;
mod grid;
mod tetromino;
mod vec2;
use crate::bag::*;
use crate::grid::*;

const WALL_KICK_OFFSETS: [i8; 2] = [-1, 1];

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
// fix speed too slow at startup
// save score -> leaderboard NOTE: very fun ! but easy to cheat
// bag preview -> next piece preview (anoying as fuck cause i have to pre-shot the next bag) or no ? if i refill when size is one
// gameover -> replay ?
// remove expects (rendererrors)
// ccw rotate

fn main() -> () {
    let mut terminal = ratatui::init();

    // setup game vars
    let mut level: u64 = 1;
    let mut score: u64 = 0;
    let mut total_lines_cleared: u64 = 0;
    let mut delta_time: Duration;
    let mut previous_time = Instant::now();
    let mut time_since_last_move = Duration::new(0, 0);
    let mut grid: Grid = [[None; GRID_WIDTH]; GRID_HEIGHT];
    let mut bag = new_bag();

    'gameloop: loop {
        delta_time = Instant::now() - previous_time;
        time_since_last_move += delta_time;
        previous_time = Instant::now();

        match update(
            &mut bag,
            &mut grid,
            &mut time_since_last_move,
            &mut level,
            &mut score,
            &mut total_lines_cleared,
        ) {
            GameEvent::GameOver => {
                ratatui::restore();
                println!("Game Over :(");
                return;
            }
            GameEvent::Tick => {}
            GameEvent::Quit => break 'gameloop,
        }
        render(&bag, grid, &mut terminal, level, score);
    }
    ratatui::restore();
    println!("Score: {}", score);
}

fn update(
    bag: &mut Bag,
    grid: &mut Grid,
    time_since_last_move: &mut Duration,
    level: &mut u64,
    score: &mut u64,
    total_lines_cleared: &mut u64,
) -> GameEvent {
    let mut next_tetromino = bag.last().expect("bag empty at start of update :/").clone();

    if event::poll(Duration::from_secs(0)).unwrap_or(false) {
        if let Ok(Event::Key(key)) = event::read() {
            match key.code {
                KeyCode::Esc => return GameEvent::Quit,
                KeyCode::Left => next_tetromino.pos.x -= 1,
                KeyCode::Right => next_tetromino.pos.x += 1,
                KeyCode::Up => next_tetromino.rotate(),
                KeyCode::Down => return hard_drop(&mut next_tetromino, bag, grid, level, score, total_lines_cleared),
                KeyCode::Char(' ') => {
                    return hard_drop(&mut next_tetromino, bag, grid, level, score, total_lines_cleared);
                }
                // vim keys
                KeyCode::Char('h') => next_tetromino.pos.x -= 1,
                KeyCode::Char('l') => next_tetromino.pos.x += 1,
                KeyCode::Char('k') => next_tetromino.rotate(),
                KeyCode::Char('j') => {
                    return hard_drop(&mut next_tetromino, bag, grid, level, score, total_lines_cleared);
                }
                _ => {}
            }
        }
    }

    // sideways collisions
    if next_tetromino.collide(grid) {
        for x in WALL_KICK_OFFSETS {
            let mut kicked_tetromino = next_tetromino.clone();
            kicked_tetromino.pos.x += x;
            if !kicked_tetromino.collide(grid) {
                // move
                *bag.last_mut().expect("bag empty on move") = kicked_tetromino.clone();
                return GameEvent::Tick; // skip gravity check for a tick
            }
        }
        return GameEvent::Tick; // skip graviry check for a tick
    }

    // move down
    let delay: Duration = get_delay_from_level(*level);
    if *time_since_last_move >= delay {
        *time_since_last_move = Duration::ZERO;
        // ground collision
        if next_tetromino.try_move_down(grid).is_err() {
            return place_down(bag, grid, level, score, 1.0, total_lines_cleared);
        }
    }

    // move
    *bag.last_mut().expect("bag empty on move") = next_tetromino.clone();
    GameEvent::Tick
}

fn get_delay_from_level(level: u64) -> Duration {
    // formula from https://tetris.wiki/Marathon
    Duration::from_secs_f64((0.8 - ((level as f64 - 1.0) * 0.007)).powf(level as f64 - 1.0))
}

fn place_down(
    bag: &mut Bag,
    grid: &mut Grid,
    level: &mut u64,
    score: &mut u64,
    score_multiplier: f32,
    total_lines_cleared: &mut u64,
) -> GameEvent {
    // place tetromino on grid
    bag.pop()
        .expect("bag empty on groud col")
        .stamp_onto(grid)
        .expect("tetromino move de-sync");

    // refill bag
    if bag.is_empty() {
        *bag = new_bag();
    }

    // https://tetris.wiki/Scoring#Recent_guideline_compatible_games
    let lines_cleared_this_frame = clear_lines(grid);
    match lines_cleared_this_frame {
        0 => {}
        1 => *score += (100.0 * *level as f32 * score_multiplier) as u64,
        2 => *score += (300.0 * *level as f32 * score_multiplier) as u64,
        3 => *score += (500.0 * *level as f32 * score_multiplier) as u64,
        4 => *score += (800.0 * *level as f32 * score_multiplier) as u64,
        _ => {} // TODO: error
    }
    // https://tetris.wiki/Marathon
    *total_lines_cleared += lines_cleared_this_frame as u64;
    if *total_lines_cleared / 10 > *level {
        *level = *total_lines_cleared / 10;
    }

    // check if the next tetromino will cause a game over
    if bag.last().expect("bag empty").collide(&grid) {
        return GameEvent::GameOver;
    }
    GameEvent::Tick
}

fn hard_drop(
    next_tetromino: &mut Tetromino,
    bag: &mut Bag,
    grid: &mut Grid,
    level: &mut u64,
    score: &mut u64,
    total_lines_cleared: &mut u64,
) -> GameEvent {
    while next_tetromino.try_move_down(grid).is_ok() {}
    *bag.last_mut().expect("bag empty on move") = next_tetromino.clone();
    place_down(bag, grid, level, score, 1.5, total_lines_cleared)
}

fn render(bag: &Bag, grid: Grid, terminal: &mut DefaultTerminal, level: u64, score: u64) -> () {
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

    let left_panel =
        Paragraph::new(CREDITS.to_owned() + &format!("\nScore: {score}\nlevel: {level}"))
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
