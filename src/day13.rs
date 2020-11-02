use super::common::*;
use super::intcode::{IntcodeComputer, IntcodeHandle};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::sync::mpsc::RecvError;

#[derive(FromPrimitive, Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum TileType {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

#[derive(Debug)]
enum CpuResult {
    Score(i64),
    Instr { r: Pos, tile_type: TileType },
}

pub fn day13() {
    let input = fs::read_to_string("inputs/day13.txt").unwrap();
    let handle = IntcodeComputer::new(&input, vec![(0, 2)]);

    let mut char_map = HashMap::new();
    char_map.insert(TileType::Empty, ' ');
    char_map.insert(TileType::Wall, '&');
    char_map.insert(TileType::Block, 'x');
    char_map.insert(TileType::Paddle, '_');
    char_map.insert(TileType::Ball, '0');
    let mut grid = Grid::new(TileType::Empty, char_map);
    let mut paddle_pos = Pos::new(0, 0);
    let mut last_score = 0;

    while let Ok(instr) = read_instr(&handle) {
        match instr {
            CpuResult::Score(score) => last_score = score,
            CpuResult::Instr {
                r: _r,
                tile_type: _tile_type,
            } => {
                grid.put(_r, _tile_type);
                if _tile_type == TileType::Paddle {
                    paddle_pos = _r;
                } else if _tile_type == TileType::Ball {
                    if paddle_pos.x < _r.x {
                        handle.tx_input.send(1).unwrap();
                    } else if paddle_pos.x > _r.x {
                        handle.tx_input.send(-1).unwrap();
                    } else {
                        handle.tx_input.send(0).unwrap();
                    }
                }
            }
        }
    }
    
    println!("The final score is {}", last_score);
}

fn read_instr(handle: &IntcodeHandle) -> Result<CpuResult, RecvError> {
    let x = handle.rx_output.recv()?;
    let y = handle.rx_output.recv()?;
    let third = handle.rx_output.recv()?;

    if x == -1 && y == 0 {
        Ok(CpuResult::Score(third))
    } else {
        let tile_type = FromPrimitive::from_i64(third).unwrap();

        Ok(CpuResult::Instr {
            r: Pos::new(x, y),
            tile_type,
        })
    }
}
