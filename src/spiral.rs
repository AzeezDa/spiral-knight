use std::ops::{Index, IndexMut};

/// We use bit operations to model threatening and occupying a board. The [[CellValue]] below can
/// be changed to u16, u32, u64, u128, and [[MAX_KNIGHTS]] to the corresponding width of the type
/// in order to get more knights (but this will use more memory).

type CellValue = u8;
pub const MAX_KNIGHTS: CellValue = 8;
pub const EMPTY: CellValue = 0;

/// A cell on a the checkboard is modelled as two bytes: the first indicates which team (colour) is
/// occupying that cell, and the second indicates which team is threatening that cell.
///
/// The occupier byte is either 0 (no occupier) or 1 at exactly one bit-index (that index is
/// occupying that cell).
///
/// The threatener byte can is a bit-vector indicating which teams are threatening that cell.
#[derive(Clone, Copy)]
pub struct Cell {
    occupier: CellValue,
    threatener: CellValue,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            occupier: EMPTY,
            threatener: EMPTY,
        }
    }

    fn occupy(&mut self, index: CellValue) {
        debug_assert!(index <= MAX_KNIGHTS);
        debug_assert!(self.can_occupy(index));
        debug_assert!(!self.unoccupiable());
        let team_bit = 1 << index;
        self.occupier = team_bit; // Occupy
        self.threatener |= team_bit; // Threaten cell
    }

    fn is_occupied(&self) -> bool {
        self.occupier != EMPTY
    }

    fn unoccupiable(&self) -> bool {
        // Occupied or threatened by >1 knight
        self.is_occupied() || self.threatener.count_ones() > 1
    }

    fn can_occupy(&self, index: CellValue) -> bool {
        debug_assert!(index <= MAX_KNIGHTS);
        let team_bit = 1 << index;
        // Cell is not occupied and either empty or threatened by a friendly piece
        !self.is_occupied() & ((self.threatener == EMPTY) | (self.threatener ^ team_bit == EMPTY))
    }

    fn threaten(&mut self, index: CellValue) {
        debug_assert!(index <= MAX_KNIGHTS);
        let team_bit = 1 << index;
        self.threatener |= team_bit;
    }

    pub fn to_colour(&self, colours: &[(u8, u8, u8)]) -> (u8, u8, u8) {
        match self.occupier {
            0b00000001 => colours[0],
            0b00000010 => colours[1],
            0b00000100 => colours[2],
            0b00001000 => colours[3],
            0b00010000 => colours[4],
            0b00100000 => colours[5],
            0b01000000 => colours[6],
            0b10000000 => colours[7],
            _ => (255, 255, 255), // Empty or unoccupiable cells are coloured white
        }
    }
}

/// Transforms a natural number to its corresponding place on the Ulam spiral (on which the pieces
/// are placed). This takes advantage of the fact that perfect squares lay on the upper left and
/// bottom right diagonals of the Ulam
/// spiral.
///
/// In summary, we find the square root of the input n to find which "ring" of the spiral need to
/// be on. Thereafter, based on which diagonal corner (called anchor) we are closest to, we add the
/// corresponding displacement based on that anchor to get the coordinate for n.
///
/// For example, if we want to compute the position for 18, we take the square root to get that 4
/// is closest square root (floored) which will lay on (-4/2, 4/2). Since 18 lay on the first half
/// of the interval [4^2, 5^2], we add (0, -4^2 - 18) = (0, -2) to the anchor corner to get the
/// corresponding coordinate. The other three cases are left to the reader as an exercise :^)
///
/// 36--35--34--33--32--31--30
///                         |
///     16--15--14--13--12  29
///     |               |   |
///     17  4---3---2   11  28
///     |   |       |   |   |
///     18  5   0---1   10  27
///     |   |           |   |
///     19  6---7---8---9   26
///     |                   |
///     20--21--22--23--24--25
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
    size: usize, // Number of rows or pixels per column
    grid: Vec<Cell>,
}

impl SpiralGrid {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            grid: vec![Cell::new(); (size + 1) * size],
        }
    }

    /// Transforms a coordinate from the standard Euclidean system with origin in the middle
    /// to an array based coordinate system where the origin is on the top left corner.
    fn transform(&self, x: i32, y: i32) -> (usize, usize) {
        (
            (self.size as i32 / 2 - y) as usize,
            (x + self.size as i32 / 2) as usize,
        )
    }

    pub fn max_n(&self) -> usize {
        self.size * self.size + self.size
    }

    pub fn within_bounds(&self, x: i32, y: i32) -> bool {
        let (x, y) = self.transform(x, y);
        (x < self.size) & (y <= self.size)
    }

    // Get the cell at the given coordinate given in standard Euclidean
    pub fn at(&self, (x, y): (i32, i32)) -> &Cell {
        let (x, y) = self.transform(x, y);
        &self[(x, y)]
    }

    // Get the cell as mutable at the given coordinate given in standard Euclidean
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
        // Can't place on unoccupiable cells: those that are threatened by more than 1
        // different-teamed pieces or those that are occupied. Thus, we skip them.
        if grid.at(ulam(n)).unoccupiable() {
            n += 1;
        } else {
            let (team_index, (delta_x, delta_y)) = turn_iterator
                .next()
                .map(|(a, (b, c))| (a as u8, (*b, *c)))
                .unwrap();

            // Find a cell to place the knight on
            for n_to_try in n..grid.max_n() {
                let coords @ (x, y) = ulam(n_to_try);
                let cell = grid.at_mut(coords);
                if cell.can_occupy(team_index) {
                    cell.occupy(team_index);

                    // Threaten cells based on movements
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
                        if grid.within_bounds(target.0, target.1) {
                            grid.at_mut(target).threaten(team_index);
                        }
                    }
                    break;
                }
            }
        }
    }

    grid
}
