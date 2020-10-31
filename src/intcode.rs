use num_bigint::{BigInt, Sign};
use phf::phf_map;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

#[derive(Debug)]
pub struct IntcodeComputer {
    memory: HashMap<usize, BigInt>,
    instr_ptr: usize,
    rel_base: BigInt,
    input: Arc<Mutex<Receiver<BigInt>>>,
    output: Arc<Mutex<Sender<BigInt>>>,
}

#[derive(Debug, Clone)]
struct OpCode {
    code: u32,
    modes: Vec<ParamMode>,
}

#[derive(Debug, Clone, Copy)]
pub enum ParamMode {
    Position,
    Immediate,
    Relative,
}

#[derive(Debug, Clone, Copy)]
pub enum RWMode {
    Read,
    Write,
}

impl IntcodeComputer {
    pub fn new(
        s: String,
        input: Arc<Mutex<Receiver<BigInt>>>,
        output: Arc<Mutex<Sender<BigInt>>>,
    ) -> JoinHandle<()> {
        thread::spawn(move || {
            let memory: HashMap<_, _> = s
                .trim()
                .split(",")
                .map(|substr| BigInt::from(substr.parse::<i64>().expect("Bad digit")))
                .enumerate()
                .collect();
            let mut cpu = IntcodeComputer {
                memory,
                instr_ptr: 0,
                rel_base: BigInt::from(0),
                input,
                output,
            };
            cpu.run();
        })
    }

    fn run(&mut self) {
        loop {
            let opcode = self.parse_opcode();
            //dbg!(&opcode);
            match opcode.code {
                1 => self.opcode1(opcode.modes),
                2 => self.opcode2(opcode.modes),
                3 => self.opcode3(opcode.modes),
                4 => self.opcode4(opcode.modes),
                5 => self.opcode5(opcode.modes),
                6 => self.opcode6(opcode.modes),
                7 => self.opcode7(opcode.modes),
                8 => self.opcode8(opcode.modes),
                9 => self.opcode9(opcode.modes),
                99 => break,
                x => panic!("Unknown opcode: {}!", x),
            }
        }
    }

    fn write(&mut self, location: usize, value: BigInt) {
        self.memory.insert(location, value);
    }

    fn read(&self, location: usize, param_mode: ParamMode, rw_mode: RWMode) -> BigInt {
        match rw_mode {
            RWMode::Read => match param_mode {
                ParamMode::Position => self.read_addr(bigint_to_usize(self.read_addr(location))),
                ParamMode::Immediate => self.read_addr(location),
                ParamMode::Relative => {
                    self.read_addr(bigint_to_usize(&self.rel_base + self.read_addr(location)))
                }
            },
            RWMode::Write => match param_mode {
                ParamMode::Position => self.read_addr(location),
                ParamMode::Relative => &self.rel_base + self.read_addr(location),
                ParamMode::Immediate => panic!("Got Immediate param mode in Write RW mode"),
            },
        }
    }

    fn read_addr(&self, location: usize) -> BigInt {
        self.memory
            .get(&location)
            .unwrap_or(&BigInt::from(0))
            .clone()
    }

    fn parse_opcode(&self) -> OpCode {
        let value = bigint_to_usize(self.read_addr(self.instr_ptr));
        OpCode::new(value)
    }

    fn opcode1(&mut self, modes: Vec<ParamMode>) {
        let val1 = self.read(self.instr_ptr + 1, modes[0], RWMode::Read);
        let val2 = self.read(self.instr_ptr + 2, modes[1], RWMode::Read);
        let pos = bigint_to_usize(self.read(self.instr_ptr + 3, modes[2], RWMode::Write));

        self.write(pos, val1 + val2);
        self.instr_ptr += *OPCODE_SIZE.get(&1).unwrap() + 1;
    }

    fn opcode2(&mut self, modes: Vec<ParamMode>) {
        let val1 = self.read(self.instr_ptr + 1, modes[0], RWMode::Read);
        let val2 = self.read(self.instr_ptr + 2, modes[1], RWMode::Read);
        let pos = bigint_to_usize(self.read(self.instr_ptr + 3, modes[2], RWMode::Write));

        self.write(pos, val1 * val2);
        self.instr_ptr += *OPCODE_SIZE.get(&2).unwrap() + 1;
    }

    fn opcode3(&mut self, modes: Vec<ParamMode>) {
        let pos = bigint_to_usize(self.read(self.instr_ptr + 1, modes[0], RWMode::Write));
        let input_value = self.input.lock().unwrap().recv().unwrap();
        self.write(pos, input_value);
        self.instr_ptr += *OPCODE_SIZE.get(&3).unwrap() + 1;
    }

    fn opcode4(&mut self, modes: Vec<ParamMode>) {
        let val = self.read(self.instr_ptr + 1, modes[0], RWMode::Read);
        self.output.lock().unwrap().send(val).unwrap();
        self.instr_ptr += *OPCODE_SIZE.get(&4).unwrap() + 1;
    }

    fn opcode5(&mut self, modes: Vec<ParamMode>) {
        let val1 = bigint_to_usize(self.read(self.instr_ptr + 1, modes[0], RWMode::Read));
        let val2 = bigint_to_usize(self.read(self.instr_ptr + 2, modes[1], RWMode::Read));

        if val1 != 0 {
            self.instr_ptr = val2;
        } else {
            self.instr_ptr += *OPCODE_SIZE.get(&5).unwrap() + 1;
        }
    }

    fn opcode6(&mut self, modes: Vec<ParamMode>) {
        let val1 = bigint_to_usize(self.read(self.instr_ptr + 1, modes[0], RWMode::Read));
        let val2 = bigint_to_usize(self.read(self.instr_ptr + 2, modes[1], RWMode::Read));

        if val1 == 0 {
            self.instr_ptr = val2;
        } else {
            self.instr_ptr += *OPCODE_SIZE.get(&6).unwrap() + 1;
        }
    }

    fn opcode7(&mut self, modes: Vec<ParamMode>) {
        let val1 = self.read(self.instr_ptr + 1, modes[0], RWMode::Read);
        let val2 = self.read(self.instr_ptr + 2, modes[1], RWMode::Read);
        let pos = bigint_to_usize(self.read(self.instr_ptr + 3, modes[2], RWMode::Write));

        if val1 < val2 {
            self.write(pos, BigInt::from(1));
        } else {
            self.write(pos, BigInt::from(0));
        }
        self.instr_ptr += *OPCODE_SIZE.get(&7).unwrap() + 1;
    }

    fn opcode8(&mut self, modes: Vec<ParamMode>) {
        let val1 = self.read(self.instr_ptr + 1, modes[0], RWMode::Read);
        let val2 = self.read(self.instr_ptr + 2, modes[1], RWMode::Read);
        let pos = bigint_to_usize(self.read(self.instr_ptr + 3, modes[2], RWMode::Write));

        if val1 == val2 {
            self.write(pos, BigInt::from(1));
        } else {
            self.write(pos, BigInt::from(0));
        }
        self.instr_ptr += *OPCODE_SIZE.get(&8).unwrap() + 1;
    }

    fn opcode9(&mut self, modes: Vec<ParamMode>) {
        let val1 = self.read(self.instr_ptr + 1, modes[0], RWMode::Read);
        self.rel_base += val1;
        self.instr_ptr += *OPCODE_SIZE.get(&9).unwrap() + 1;
    }
}

fn bigint_to_usize(bigint: BigInt) -> usize {
    let (sign, digits) = bigint.to_u32_digits();
    if sign == Sign::Minus {
        panic!("Negative sign converting BigInt {}", bigint);
    } else if digits.len() > 1 {
        panic!("Too many digits on BigInt {}", bigint);
    } else if digits.len() == 0 {
        0
    } else {
        digits[0] as usize
    }
}

static OPCODE_SIZE: phf::Map<u32, usize> = phf_map! {
    1u32 => 3,
    2u32 => 3,
    3u32 => 1,
    4u32 => 1,
    5u32 => 2,
    6u32 => 2,
    7u32 => 3,
    8u32 => 3,
    9u32 => 1,
    99u32 => 0,
};

impl OpCode {
    pub fn new(value: usize) -> OpCode {
        let code = (value % 100) as u32;
        let size = *OPCODE_SIZE.get(&code).unwrap();

        let mut modes = Vec::new();
        let mut codes = value / 100;

        for _ in 0..size {
            let param_mode = match codes % 10 {
                0 => ParamMode::Position,
                1 => ParamMode::Immediate,
                2 => ParamMode::Relative,
                x => panic!("Unknown param mode {}", x),
            };

            modes.push(param_mode);
            codes /= 10;
        }

        OpCode { code, modes }
    }
}
