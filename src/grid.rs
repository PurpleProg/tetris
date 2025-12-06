use ratatui::style::Color;

pub const GRID_WIDTH: usize = 10;
pub const GRID_HEIGHT: usize = 20;
pub type Grid = [[Option<Color>; GRID_WIDTH]; GRID_HEIGHT];

pub fn clear_lines(grid: &mut Grid) -> u8 {
    let mut count = 0;
    while grid
        .iter()
        .any(|line| line.iter().all(|cell| cell.is_some()))
    {
        clear_one_line(grid);
        count += 1;
    }
    count
}

fn clear_one_line(grid: &mut Grid) -> () {
    let Some(first_full_line) = grid
        .iter()
        .position(|line| line.iter().all(|cell| cell.is_some()))
    else {
        return;
    };
    for i in (1..=first_full_line).rev() {
        grid[i] = grid[i - 1];
    }
}
