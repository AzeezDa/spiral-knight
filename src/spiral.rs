use std::{
    ops::{Index, IndexMut},
};

type CellValue = u8;
const OCCUPIED: CellValue = 1;
pub const MAX_KNIGHTS: CellValue = 7;
const EMPTY: CellValue = 0;

#[derive(Clone, Copy)]
pub struct Cell {
    occupier: CellValue,
    cell: CellValue,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            occupier: 0,
            cell: 0,
        }
    }

    fn occupy(&mut self, index: CellValue) {
        debug_assert!(index <= MAX_KNIGHTS);
        debug_assert!(self.can_occupy(index));
        debug_assert!(!self.unoccupiable());
        let team = Self::index_to_team(index);
        self.occupier = team;
        self.cell |= OCCUPIED | team; // Occupy and paint cell
    }

    fn is_occupied(&self) -> bool {
        self.cell & OCCUPIED == OCCUPIED
    }

    fn unoccupiable(&self) -> bool {
        // Occupied or threatened by >1 knight
        self.is_occupied() || self.cell.count_ones() > 1
    }

    fn can_occupy(&self, index: CellValue) -> bool {
        debug_assert!(index <= MAX_KNIGHTS);
        let team = Self::index_to_team(index);
        !self.is_occupied() & ((self.cell == EMPTY) | (self.cell ^ team == EMPTY))
    }

    fn threaten(&mut self, index: CellValue) {
        debug_assert!(index <= MAX_KNIGHTS);
        let team = Self::index_to_team(index);
        self.cell |= team;
    }

    pub fn to_colour(&self, colours: &[(u8, u8, u8)]) -> (u8, u8, u8) {
        match self.occupier {
            0b00000010 => colours[0], // Red
            0b00000100 => colours[1], // Black
            0b00001000 => colours[2], // Cyan
            0b00010000 => colours[3], // Green
            0b00100000 => colours[4], // Violet
            0b01000000 => colours[5], // Orange
            0b10000000 => colours[6], // Blue
            _ => (255, 255, 255),     // White
        }
    }

    fn index_to_team(index: CellValue) -> CellValue {
        debug_assert!(index <= MAX_KNIGHTS);
        1 << (index + 1)
    }
}

fn ulam(n: usize) -> (i32, i32) {
    let n = n as i32;
    let sqrtn = (n as f64).sqrt() as i32;
    let sqrtn_2 = sqrtn / 2;
    let lower_square = sqrtn * sqrtn;
    let delta_square = n - lower_square;
    if sqrtn % 2 == 0 {
        let mut anchor = (-sqrtn_2, sqrtn_2);
        if delta_square <= sqrtn {
            anchor.1 -= delta_square;
        } else {
            anchor.1 -= sqrtn;
            anchor.0 += delta_square - sqrtn;
        }
        anchor
    } else {
        let mut anchor = (1 + sqrtn_2, -sqrtn_2);
        if delta_square <= sqrtn {
            anchor.1 += delta_square;
        } else {
            anchor.1 += sqrtn;
            anchor.0 -= delta_square - sqrtn;
        }

        anchor
    }
}

pub struct SpiralGrid {
    size: usize,
    grid: Vec<Cell>,
}

impl SpiralGrid {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            grid: vec![Cell::new(); (size + 1) * size],
        }
    }

    fn transform(&self, x: i32, y: i32) -> (usize, usize) {
        (
            (self.size as i32 / 2 - y) as usize,
            (x + self.size as i32 / 2) as usize,
        )
    }

    pub fn max_n(&self) -> usize {
        self.size * self.size + self.size
    }

    pub fn with_in_bounds(&self, x: i32, y: i32) -> bool {
        let (x, y) = self.transform(x, y);
        (x < self.size) & (y <= self.size)
    }

    pub fn at(&self, (x, y): (i32, i32)) -> &Cell {
        let (x, y) = self.transform(x, y);
        &self[(x, y)]
    }

    pub fn at_mut(&mut self, (x, y): (i32, i32)) -> &mut Cell {
        let (x, y) = self.transform(x, y);
        &mut self[(x, y)]
    }

    pub fn grid(&self) -> &[Cell] {
        &self.grid
    }
}

impl IndexMut<(usize, usize)> for SpiralGrid {
    fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut Self::Output {
        &mut self.grid[r * (self.size + 1) + c]
    }
}

impl Index<(usize, usize)> for SpiralGrid {
    type Output = Cell;

    fn index(&self, (r, c): (usize, usize)) -> &Self::Output {
        &self.grid[r * (self.size + 1) + c]
    }
}

pub fn place_knights(size: usize, knights: &[(i32, i32)]) -> SpiralGrid {
    debug_assert!(knights.len() <= MAX_KNIGHTS as usize);
    let mut grid = SpiralGrid::new(size);
    let mut turn_iterator = knights.iter().enumerate().cycle();
    let mut n = 0;

    while n < grid.max_n() {
        if grid.at(ulam(n)).unoccupiable() {
            n += 1;
        } else {
            let (knight_index, (delta_x, delta_y)) = turn_iterator
                .next()
                .map(|(a, (b, c))| (a as u8, (*b, *c)))
                .unwrap();
            for n_to_try in n..grid.max_n() {
                let coords @ (x, y) = ulam(n_to_try);
                let cell = grid.at_mut(coords);
                if cell.can_occupy(knight_index) {
                    cell.occupy(knight_index);
                    for (delta_x, delta_y) in [
                        (delta_x, delta_y),
                        (-delta_x, delta_y),
                        (delta_x, -delta_y),
                        (-delta_x, -delta_y),
                        (delta_y, delta_x),
                        (-delta_y, delta_x),
                        (delta_y, -delta_x),
                        (-delta_y, -delta_x),
                    ] {
                        let target = (x + delta_x, y + delta_y);
                        if grid.with_in_bounds(target.0, target.1) {
                            grid.at_mut(target).threaten(knight_index);
                        }
                    }
                    break;
                }
            }
        }
    }

    grid
}
