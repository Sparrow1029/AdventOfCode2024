use std::{fmt::Debug, vec::IntoIter};

use grid::Grid;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn tuple(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    pub fn in_bounds<T>(&self, grid: &Grid<T>) -> bool {
        self.x >= 0 && self.x < grid.cols() as isize && self.y >= 0 && self.y < grid.rows() as isize
    }
    pub fn diagonal_neighbors<T>(&self, grid: &Grid<T>) -> [Option<T>; 4]
    where
        T: Clone + Default,
    {
        [
            (self.x - 1, self.y - 1),
            (self.x + 1, self.y - 1),
            (self.x + 1, self.y + 1),
            (self.x - 1, self.y + 1),
        ]
        .map(|(x, y)| {
            if Point::from((x, y)).in_bounds(grid) {
                Some(grid.get(y, x).expect("doesn't exist").clone())
            } else {
                None
            }
        })
    }
}

impl From<(isize, isize)> for Point {
    fn from(value: (isize, isize)) -> Self {
        Self::new(value.0, value.1)
    }
}

pub fn down_right(x: usize, y: usize) -> (usize, usize) {
    (x + 1, y + 1)
}

pub fn down_left(x: usize, y: usize) -> (usize, usize) {
    (x.saturating_sub(1), y + 1)
}

pub fn iter_diag_nesw<T>(grid: &Grid<T>) -> IntoIter<Vec<T>>
where
    T: Default + Copy,
{
    let mut diagonals = vec![];
    // lower half of grid
    for y in 0..grid.rows() {
        let mut diag = vec![];
        let (mut cur_x, mut cur_y) = (grid.cols() - 1, y);
        while let Some(c) = grid.get(cur_y, cur_x) {
            diag.push(c.to_owned());
            (cur_x, cur_y) = down_left(cur_x, cur_y);
        }
        diagonals.push(diag);
    }
    // get rest
    for x in 0..grid.cols() - 1 {
        let mut diag = vec![];
        let (mut cur_x, mut cur_y) = (x, 0);
        while let Some(c) = grid.get(cur_y, cur_x) {
            diag.push(c.to_owned());
            if cur_x == 0 {
                break;
            }
            (cur_x, cur_y) = down_left(cur_x, cur_y);
        }
        diagonals.push(diag);
    }
    diagonals.into_iter()
}

pub fn iter_diag_nwse<T>(grid: &Grid<T>) -> IntoIter<Vec<T>>
where
    T: Default + Copy,
{
    let mut diagonals = vec![];
    // get all lower left half of grid
    for y in 0..grid.rows() {
        let mut diag = vec![];
        let (mut cur_x, mut cur_y) = (0, y);
        while let Some(c) = grid.get(cur_y, cur_x) {
            diag.push(c.to_owned());
            (cur_x, cur_y) = down_right(cur_x, cur_y);
        }
        diagonals.push(diag);
    }
    // get rest
    for x in 1..grid.cols() {
        let mut diag = vec![];
        let (mut cur_x, mut cur_y) = (x, 0);
        while let Some(c) = grid.get(cur_y, cur_x) {
            diag.push(c.to_owned());
            (cur_x, cur_y) = down_right(cur_x, cur_y);
        }
        diagonals.push(diag);
    }
    diagonals.into_iter()
}

pub struct FlatGrid<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

impl<T> FlatGrid<T>
where
    T: Copy + Clone + Debug + PartialEq + Eq + std::fmt::Display,
{
    pub fn new(width: usize, height: usize, data: Option<Vec<T>>) -> FlatGrid<T> {
        let data = if let Some(data) = data {
            data
        } else {
            Vec::with_capacity(width * height)
        };
        FlatGrid {
            width,
            height,
            data,
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> &T {
        &self.data[y * self.width + x]
    }

    fn set_cell(&mut self, x: usize, y: usize, data: T) {
        self.data[y * self.width + x] = data.to_owned();
    }

    pub fn replace_cell(&mut self, x: usize, y: usize, data: T) -> T {
        if self.in_bounds(x, y) {
            let orig = *self.get_cell(x, y);
            self.set_cell(x, y, data);
            orig
        } else {
            panic!("oh no")
        }
    }

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn rows(&self) -> std::ops::Range<usize> {
        0..self.width
    }

    pub fn columns(&self) -> std::ops::Range<usize> {
        0..self.height
    }
}

impl<T> std::fmt::Display for FlatGrid<T>
where
    T: Clone + Copy + Debug + PartialEq + Eq + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_rep = (0..self.height)
            .map(|y| {
                let mut row = (0..self.width)
                    .map(|x| format!("{} ", self.get_cell(x, y)))
                    .collect_vec()
                    .concat();
                row.push('\n');
                row
            })
            .collect_vec()
            .concat();
        write!(f, "{}", str_rep)
    }
}
