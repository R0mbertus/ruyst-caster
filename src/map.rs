pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 8;

// constant map for now
const MAP: [[u8; WIDTH]; HEIGHT] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 0, 0, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 0, 0, 1, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
];

pub fn wall_point(x: usize, y: usize) -> bool {
    match MAP.get(y) {
        Some(row) =>    *row.get(x).unwrap() != 0,
        None =>         true,
    }
}

pub fn block_size(height: f64, width: f64) -> (f64, f64) {
    (height / (HEIGHT as f64), width / (WIDTH as f64))
}
