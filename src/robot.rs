use super::common::*;
use super::intcode::IntcodeHandle;
use std::collections::{HashMap, HashSet};
use std::sync::mpsc::RecvError;

#[derive(Debug)]
pub struct Robot {
    grid: Grid<Color>,
    history: HashSet<Pos>,
    r: Pos,
    dir: Pos,
    cpu: IntcodeHandle,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Color {
    Black,
    White,
}

impl Robot {
    pub fn new(cpu: IntcodeHandle) -> Robot {
        let mut char_map = HashMap::new();
        char_map.insert(Color::Black, ' ');
        char_map.insert(Color::White, '*');

        let mut grid = Grid::new(Color::Black, char_map);
        grid.put(Pos::new(0, 0), Color::White);
        Robot {
            grid,
            history: HashSet::new(),
            r: Pos::new(0, 0),
            dir: Pos::new(0, 1),
            cpu,
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.read_color() {
                Color::Black => self.cpu.tx_input.send(0).unwrap(),
                Color::White => self.cpu.tx_input.send(1).unwrap(),
            }

            let (paint, turn) = match self.read_instr() {
                Ok((paint, turn)) => (paint, turn),
                Err(_) => {
                    println!("CPU exited, exiting robot");
                    break;
                }
            };

            let color = match paint {
                0 => Color::Black,
                1 => Color::White,
                x => panic!("Received unknown code from CPU {}", x),
            };

            self.history.insert(self.r);
            self.grid.put(self.r, color);

            let dir = match turn {
                0 => Dir::Left,
                1 => Dir::Right,
                x => panic!("Received unknown code from CPU {}", x),
            };

            self.dir.rotate(dir);
            self.r = self.r + self.dir;
        }

        println!("Painted {} tiles", self.history.len());
        self.print_grid();
    }

    fn read_instr(&self) -> Result<(i64, i64), RecvError> {
        let turn = self.cpu.rx_output.recv()?;
        let paint = self.cpu.rx_output.recv()?;

        Ok((turn, paint))
    }

    fn read_color(&self) -> Color {
        *self.grid.get(&self.r)
    }

    fn print_grid(&self) {
        self.grid.print_array();
        //let mut max_x = std::i32::MIN;
        //let mut min_x = std::i32::MAX;
        //let mut max_y = std::i32::MIN;
        //let mut min_y = std::i32::MAX;

        //self.grid.keys().for_each(|pos| {
            //if pos.x > max_x {
                //max_x = pos.x;
            //}
            //if pos.x < min_x {
                //min_x = pos.x;
            //}
            //if pos.y > max_y {
                //max_y = pos.y;
            //}
            //if pos.y < min_y {
                //min_y = pos.y
            //}
        //});

        //let x_size = max_x + min_x.abs() + 1;
        //let y_size = max_y + min_y.abs() + 1;

        //let mut array_grid = vec![vec![' '; 1 + x_size as usize]; 1 + y_size as usize];

        //for (pos, color) in &self.grid {
            //let i = (pos.y + min_y.abs()) as usize;
            //let j = (pos.x + min_x.abs()) as usize;
            //let c = match color {
                //Color::Black => ' ',
                //Color::White => '*'
            //};

            //array_grid[i][j] = c;
        //}

        //for i in (0..y_size).rev() {
            //for j in 0..x_size {
                //print!("{}", array_grid[i as usize][j as usize]);
            //}
            //println!();
        //}
    }
}
