use ratatui::style::Color;

pub const GRID_WIDTH: usize = 10;
pub const GRID_HEIGHT: usize = 20;
pub type Grid = [[Option<Color>; GRID_WIDTH]; GRID_HEIGHT];

pub fn clear_lines(grid: &mut Grid) -> () {
    let line_index: Option<usize> = grid
        .iter()
        .enumerate()
        .position(|(_, line)| line.iter().all(|b| b.is_some()));
    let Some(last_filled) = line_index else {
        return;
    };
    for i in (1..=last_filled).rev() {
        grid[i] = grid[i - 1];
    }
}
