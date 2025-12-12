use crossterm::event::{self, Event, KeyCode};
use leaderboard::Entry;
use ratatui::{
    DefaultTerminal,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols,
    widgets::{Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tetromino::Tetromino;
use users::get_current_username;

mod bag;
mod grid;
mod leaderboard;
mod tetromino;
mod vec2;
use crate::bag::*;
use crate::grid::*;
use crate::leaderboard::*;

const TARGET_FPS: u8 = 60;
// NOTE: need 2 and -2 for the red I,
// but that would make some tetromino clip through some thin "walls"
const WALL_KICK_OFFSETS: [i8; 2] = [-1, 1];
const CREDITS: &str = "Tetris
Author : mphippen
Source : https://github.com/PurpleProg/tetris

rust > C";

#[derive(Debug)]
enum GameEvent {
    GameOver,
    Quit,
    Tick,
}

struct GameContext {
    level: u64,
    score: u64,
    username: String,
    total_lines_cleared: u64,
    grid: Grid,
    bag: Bag,
    leaderboard: LeaderBoard,
}

// TODO:
// fix speed too slow at startup
// save score -> leaderboard NOTE: very fun ! but easy to cheat
// bag preview -> next piece preview (anoying as fuck cause i have to pre-shot the next bag) or no ? if i refill when size is one
// gameover -> replay ?
// remove expects (rendererrors)
// ccw rotate
// preview, hold

fn main() -> () {
    let mut terminal = ratatui::init();

    // setup game vars
    let mut game_context = GameContext {
        level: 1,
        score: 0,
        username: get_current_username()
            .unwrap_or("User not found".into())
            .into_string()
            .expect("error converting OsString to String"),
        total_lines_cleared: 0,
        grid: [[None; GRID_WIDTH]; GRID_HEIGHT],
        bag: new_bag(),
        leaderboard: LeaderBoard::load(".scores"),
    };
    if game_context
        .leaderboard
        .get_entry(&game_context.username)
        .is_none()
    {
        game_context
            .leaderboard
            .add_entry(Entry::new(&game_context));
    }

    let mut delta_time: Duration;
    let mut previous_time = Instant::now();
    let mut time_since_last_move = Duration::new(0, 0);

    let tick_rate: Duration = Duration::from_secs_f32(1.0 / TARGET_FPS as f32);

    let mut frame_time: Duration = Duration::new(0, 0);

    'gameloop: loop {
        delta_time = Instant::now() - previous_time;
        time_since_last_move += delta_time;
        frame_time += delta_time;
        previous_time = Instant::now();
        if frame_time < tick_rate {
            std::thread::sleep(tick_rate - frame_time);
            frame_time = Duration::ZERO;
            continue 'gameloop;
        }

        match update(&mut game_context, &mut time_since_last_move) {
            GameEvent::GameOver => {
                // NOTE: gameover logic here ?
                // could use it's own function
                ratatui::restore();
                println!("Game Over :(");
                game_context.leaderboard.save(".scores");
                println!(
                    "Score: {}, level: {}",
                    game_context.score, game_context.level
                );
                return;
            }
            GameEvent::Tick => {}
            GameEvent::Quit => break 'gameloop,
        }
        // let _ = game_context
        //     .leaderboard
        //     .update_entry(
        //         &game_context.username,
        //         game_context.score,
        //         game_context.level,
        //     )
        //     .expect("entry not found for update");
        render(&game_context, &mut terminal);
    }
    // NOTE: quit logic here ?
    // could use it's own function too
    ratatui::restore();
    let _ = game_context
        .leaderboard
        .update_entry(
            &game_context.username,
            game_context.score,
            game_context.level,
        )
        .expect("entry not found for update");
    game_context.leaderboard.save(".scores");
    println!(
        "Score: {}, level: {}",
        game_context.score, game_context.level
    );
}

fn update(game_context: &mut GameContext, time_since_last_move: &mut Duration) -> GameEvent {
    let mut next_tetromino = game_context
        .bag
        .last()
        .expect("bag empty at start of update :/")
        .clone();

    if event::poll(Duration::from_secs(0)).unwrap_or(false) {
        if let Ok(Event::Key(key)) = event::read() {
            match key.code {
                KeyCode::Esc => return GameEvent::Quit,
                KeyCode::Left => next_tetromino.pos.x -= 1,
                KeyCode::Right => next_tetromino.pos.x += 1,
                KeyCode::Up => next_tetromino.rotate(), // hard drop
                KeyCode::Down => return hard_drop(&mut next_tetromino, game_context), // soft drop
                KeyCode::Char(' ') => return hard_drop(&mut next_tetromino, game_context),
                // z cw
                // x 180
                // c ccw
                // s hold piece
                // vim keys
                KeyCode::Char('h') => next_tetromino.pos.x -= 1,
                KeyCode::Char('l') => next_tetromino.pos.x += 1,
                KeyCode::Char('k') => next_tetromino.rotate(),
                KeyCode::Char('j') => return hard_drop(&mut next_tetromino, game_context),
                _ => {}
            }
        }
    }

    // sideways collisions
    if next_tetromino.collide(&game_context.grid) {
        for x in WALL_KICK_OFFSETS {
            let mut kicked_tetromino = next_tetromino.clone();
            kicked_tetromino.pos.x += x;
            if !kicked_tetromino.collide(&game_context.grid) {
                // move
                *game_context.bag.last_mut().expect("bag empty on move") = kicked_tetromino.clone();
                return GameEvent::Tick; // skip gravity check for a tick
            }
        }
        return GameEvent::Tick; // skip graviry check for a tick
    }

    // move down
    let delay: Duration = get_delay_from_level(game_context.level);
    if *time_since_last_move >= delay {
        *time_since_last_move = Duration::ZERO;
        // ground collision
        if next_tetromino.try_move_down(&game_context.grid).is_err() {
            return place_down(game_context, 1.0);
        }
    }

    // move
    *game_context.bag.last_mut().expect("bag empty on move") = next_tetromino.clone();
    GameEvent::Tick
}

fn get_delay_from_level(level: u64) -> Duration {
    // formula from https://tetris.wiki/Marathon
    Duration::from_secs_f64((0.8 - ((level as f64 - 1.0) * 0.007)).powf(level as f64 - 1.0))
}

fn place_down(game_context: &mut GameContext, score_multiplier: f32) -> GameEvent {
    // place tetromino on grid
    game_context
        .bag
        .pop()
        .expect("bag empty on groud col")
        .stamp_onto(&mut game_context.grid)
        .expect("tetromino move de-sync");

    // refill bag
    if game_context.bag.is_empty() {
        game_context.bag = new_bag();
    }

    // https://tetris.wiki/Scoring#Recent_guideline_compatible_games
    let lines_cleared_this_frame = clear_lines(&mut game_context.grid);
    match lines_cleared_this_frame {
        0 => {}
        1 => game_context.score += (100.0 * game_context.level as f32 * score_multiplier) as u64,
        2 => game_context.score += (300.0 * game_context.level as f32 * score_multiplier) as u64,
        3 => game_context.score += (500.0 * game_context.level as f32 * score_multiplier) as u64,
        4 => game_context.score += (800.0 * game_context.level as f32 * score_multiplier) as u64,
        _ => {} // TODO: error
    }
    // https://tetris.wiki/Marathon
    game_context.total_lines_cleared += lines_cleared_this_frame as u64;
    if game_context.total_lines_cleared / 10 > game_context.level {
        game_context.level = game_context.total_lines_cleared / 10;
    }

    // check if the next tetromino will cause a game over
    if game_context
        .bag
        .last()
        .expect("bag empty")
        .collide(&game_context.grid)
    {
        return GameEvent::GameOver;
    }
    GameEvent::Tick
}

fn hard_drop(next_tetromino: &mut Tetromino, game_context: &mut GameContext) -> GameEvent {
    while next_tetromino.try_move_down(&game_context.grid).is_ok() {}
    *game_context.bag.last_mut().expect("bag empty on move") = next_tetromino.clone();
    place_down(game_context, 1.0)
}

fn render(game_context: &GameContext, terminal: &mut DefaultTerminal) -> () {
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

    let left_panel = Paragraph::new(
        CREDITS.to_owned()
            + &format!(
                "\nScore: {}\nlevel: {}",
                game_context.score, game_context.level
            ),
    )
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

    let right_panel = Paragraph::new(game_context.leaderboard.to_string()).block(
        Block::default()
            .borders(Borders::ALL)
            .border_set(symbols::border::ROUNDED)
            .border_style(Style::default().fg(ratatui::style::Color::DarkGray))
            .title(" 42 lyon leaderboard ")
            .title_alignment(Alignment::Center)
            .title_style(Style::default().fg(Color::White)),
    );

    let playfield = Block::default()
        .borders(Borders::ALL)
        .border_set(symbols::border::ROUNDED)
        .border_style(Style::default().fg(ratatui::style::Color::DarkGray))
        .title(" Playfield ")
        .title_alignment(Alignment::Center)
        .title_style(Style::default().fg(Color::White));

    // create a new temp grid that hold the current tetromino
    let mut grid_with_tetromino = game_context.grid.clone();
    game_context
        .bag
        .last()
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
