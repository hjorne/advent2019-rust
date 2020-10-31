use phf::phf_map;
use std::sync::{Mutex, Arc};
use std::sync::mpsc::{Sender, Receiver};

#[derive(Debug)]
pub struct IntcodeComputer {
    memory: Vec<i64>,
    instr_ptr: usize,
    input: Arc<Mutex<Receiver<i64>>>,
    output: Arc<Mutex<Sender<i64>>>,
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
}

impl IntcodeComputer {
    pub fn new(s: String, input: Arc<Mutex<Receiver<i64>>>, output: Arc<Mutex<Sender<i64>>>) -> IntcodeComputer {
        let memory = s
            .trim()
            .split(",")
            .map(|substr| substr.parse::<i64>().expect("Bad digit"))
            .collect();
        IntcodeComputer {
            memory,
            instr_ptr: 0,
            input,
            output
        }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.parse_opcode();
            match opcode.code {
                1 => self.opcode1(opcode.modes),
                2 => self.opcode2(opcode.modes),
                3 => self.opcode3(),
                4 => self.opcode4(opcode.modes),
                5 => self.opcode5(opcode.modes),
                6 => self.opcode6(opcode.modes),
                7 => self.opcode7(opcode.modes),
                8 => self.opcode8(opcode.modes),
                99 => break,
                x => panic!("Unknown opcode: {}!", x),
            }
        }
    }

    pub fn write(&mut self, location: usize, value: i64) {
        self.memory[location] = value;
    }

    pub fn read(&self, location: usize, mode: Option<ParamMode>) -> i64 {
        match mode {
            Some(mode) => match mode {
                ParamMode::Position => self.memory[self.memory[location] as usize],
                ParamMode::Immediate => self.memory[location],
            },
            None => self.memory[location],
        }
    }

    fn parse_opcode(&self) -> OpCode {
        let value = self.memory[self.instr_ptr];
        OpCode::new(value)
    }

    fn opcode1(&mut self, modes: Vec<ParamMode>) {
        let val1 = self.read(self.instr_ptr + 1, Some(modes[0]));
        let val2 = self.read(self.instr_ptr + 2, Some(modes[1]));
        let pos = self.read(self.instr_ptr + 3, None);

        self.write(pos as usize, val1 + val2);
        self.instr_ptr += *OPCODE_SIZE.get(&1).unwrap() + 1;
    }

    fn opcode2(&mut self, modes: Vec<ParamMode>) {
        let val1 = self.read(self.instr_ptr + 1, Some(modes[0]));
        let val2 = self.read(self.instr_ptr + 2, Some(modes[1]));
        let pos = self.read(self.instr_ptr + 3, None);

        self.write(pos as usize, val1 * val2);
        self.instr_ptr += *OPCODE_SIZE.get(&2).unwrap() + 1;
    }

    fn opcode3(&mut self) {
        let pos = self.read(self.instr_ptr + 1, None);
        let input_value = self.input.lock().unwrap().recv().unwrap();
        self.write(pos as usize, input_value);
        self.instr_ptr += *OPCODE_SIZE.get(&3).unwrap() + 1;
    }

    fn opcode4(&mut self, modes: Vec<ParamMode>) {
        let val = self.read(self.instr_ptr + 1, Some(modes[0]));
        self.output.lock().unwrap().send(val).unwrap();
        self.instr_ptr += *OPCODE_SIZE.get(&4).unwrap() + 1;
    }

    fn opcode5(&mut self, modes: Vec<ParamMode>) {
        let val1 = self.read(self.instr_ptr + 1, Some(modes[0]));
        let val2 = self.read(self.instr_ptr + 2, Some(modes[1]));

        if val1 != 0 {
            self.instr_ptr = val2 as usize;
        } else {
            self.instr_ptr += *OPCODE_SIZE.get(&5).unwrap() + 1;
        }
    }

    fn opcode6(&mut self, modes: Vec<ParamMode>) {
        let val1 = self.read(self.instr_ptr + 1, Some(modes[0]));
        let val2 = self.read(self.instr_ptr + 2, Some(modes[1]));

        if val1 == 0 {
            self.instr_ptr = val2 as usize;
        } else {
            self.instr_ptr += *OPCODE_SIZE.get(&6).unwrap() + 1;
        }
    }

    fn opcode7(&mut self, modes: Vec<ParamMode>) {
        let val1 = self.read(self.instr_ptr + 1, Some(modes[0]));
        let val2 = self.read(self.instr_ptr + 2, Some(modes[1]));
        let pos = self.read(self.instr_ptr + 3, None) as usize;

        if val1 < val2 {
            self.write(pos, 1);
        } else {
            self.write(pos, 0);
        }
        self.instr_ptr += *OPCODE_SIZE.get(&7).unwrap() + 1;
    }

    fn opcode8(&mut self, modes: Vec<ParamMode>) {
        let val1 = self.read(self.instr_ptr + 1, Some(modes[0]));
        let val2 = self.read(self.instr_ptr + 2, Some(modes[1]));
        let pos = self.read(self.instr_ptr + 3, None) as usize;

        if val1 == val2 {
            self.write(pos, 1);
        } else {
            self.write(pos, 0);
        }
        self.instr_ptr += *OPCODE_SIZE.get(&8).unwrap() + 1;
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
    99u32 => 0,
};

impl OpCode {
    pub fn new(value: i64) -> OpCode {
        let code = (value % 100) as u32;
        let size = *OPCODE_SIZE.get(&code).unwrap();

        let mut modes = Vec::new();
        let mut codes = value / 100;

        for _ in 0..size {
            let param_mode = match codes % 10 {
                0 => ParamMode::Position,
                1 => ParamMode::Immediate,
                x => panic!("Unknown param mode {}", x),
            };

            modes.push(param_mode);
            codes /= 10;
        }

        OpCode { code, modes }
    }
}
