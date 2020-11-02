use derive_more::{Add, Constructor};
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Add, Constructor)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    Left,
    Right,
}

impl Pos {
    pub fn rotate(&mut self, dir: Dir) {
        match dir {
            Dir::Left => {
                let tmp = self.x;
                self.x = -self.y;
                self.y = tmp;
            }
            Dir::Right => {
                let tmp = self.x;
                self.x = self.y;
                self.y = -tmp;
            }
        };
    }
}

#[derive(Debug)]
pub struct Grid<T> {
    pub grid: HashMap<Pos, T>,
    char_map: HashMap<T, char>,
    default: T
}

impl<T> Grid<T>
where
    T: Hash + Eq + PartialEq + Debug,
{
    pub fn new(default: T, char_map: HashMap<T, char>) -> Grid<T> {
        Grid {
            grid: HashMap::new(),
            char_map,
            default
        }
    }

    pub fn put(&mut self, r: Pos, v: T) {
        self.grid.insert(r, v);
    }

    pub fn get(&self, r: &Pos) -> &T {
        self.grid.get(r).unwrap_or(&self.default)
    }

    pub fn bounds(&self) -> ((i64, i64), (i64, i64)) {
        let mut max_x = std::i64::MIN;
        let mut min_x = std::i64::MAX;
        let mut max_y = std::i64::MIN;
        let mut min_y = std::i64::MAX;

        self.grid.keys().for_each(|pos| {
            if pos.x > max_x {
                max_x = pos.x;
            }
            if pos.x < min_x {
                min_x = pos.x;
            }
            if pos.y > max_y {
                max_y = pos.y;
            }
            if pos.y < min_y {
                min_y = pos.y
            }
        });

        ((min_x, max_x), (min_y, max_y))
    }

    pub fn to_array(&self) -> Vec<Vec<char>> {
        let ((min_x, max_x), (min_y, max_y)) = self.bounds();
        let x_size = (max_x.abs() + min_x.abs() + 1) as usize;
        let y_size = (max_y.abs() + min_y.abs() + 1) as usize;

        let mut array_grid = vec![vec![' '; x_size]; y_size];

        for (pos, val) in &self.grid {
            let i = (pos.y + min_y.abs()) as usize;
            let j = (pos.x - min_x.abs()) as usize;

            let c = self.char_map.get(val).expect("No char type for value");

            array_grid[i][j] = *c;
        }

        array_grid
    }

    pub fn print_array(&self) {
        let array = self.to_array();
        for i in 0..array.len() {
            for j in 0..array[i].len()  {
                print!("{}", array[i as usize][j as usize]);
            }
            println!();
        }
    }
}
