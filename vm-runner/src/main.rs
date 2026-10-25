use std::io::{self, BufRead};
use std::process::exit;

pub enum REG_ABI {
    ZERO = 0, // constant zero
    RA = 1,   // return address
    SP = 2,   // stack pointer
    GP = 3,   // global pointer
    T0 = 4,   // temps
    T1 = 5,
    T2 = 6,
    T3 = 7,
    S0 = 8, // saves
    S1 = 9,
    S2 = 10,
    S3 = 11,
    A0 = 12, // arguments
    A1 = 13,
    A2 = 14,
    T6 = 15, // temps
}

// Instruction: [bits 4-7 arg1_reg][bits 0-3 arg0_reg], [bits 4-7 dist_reg][bits 0-3 Opcode]
#[repr(u8)]
pub enum OP {
    MOVI = 0b0000,
    NOT = 0b0001,
    AND = 0b0010,
    OR = 0b0011,
    XOR = 0b0100,
    SHL = 0b0101,
    SHR = 0b0110,
    ADD = 0b0111,
    SUB = 0b1000,
    MUL = 0b1001,
    DIV = 0b1010,
    LOADSETMEM = 0b1011,
    SYSCALL = 0b1100,
    BLT = 0b1101,
    BGT = 0b1110,
    JMP = 0b1111,
}

/* Syscalls
 * Halt: reg A0 == 1
 * Grow heap A0 == 2, A1 = amt
 * Pop heap A0 == 3
 * Puts A0 == 4, A1 = start_ptr, A2 = len
 * Cin A0 == 5, A1 = start_ptr, A2 = len
 * TODO! FS IO
 */

fn main() {
    let mem_size: usize = 16; // From compiled bytecode file
    let mut mem: Vec<u8> = Vec::with_capacity(mem_size);
    mem.resize(mem_size, 0);
    let mut pc = 0;
    let mut regs: [u64; 16] = [0_u64; 16];

    loop {
        let inst_part0: u8 = mem[pc];
        let inst_part1: u8 = mem[pc + 1];
        let op: u8 = inst_part0 & 0b11110000;
        let dist_reg: usize = (inst_part0 & 0b00001111) as usize;
        let arg0_reg: usize = (inst_part1 & 0b11110000) as usize;
        let arg1_reg: usize = (inst_part1 & 0b00001111) as usize;

        unsafe {
            match op {
                0 => regs[dist_reg] = regs[arg0_reg] & inst_part1 as u64,
                1 => regs[dist_reg] = !regs[arg0_reg],
                2 => regs[dist_reg] = regs[arg0_reg] & regs[arg1_reg],
                3 => regs[dist_reg] = regs[arg0_reg] | regs[arg1_reg],
                4 => regs[dist_reg] = regs[arg0_reg] ^ regs[arg1_reg],
                5 => regs[dist_reg] = regs[arg0_reg] << regs[arg1_reg],
                6 => regs[dist_reg] = regs[arg0_reg] >> regs[arg1_reg],
                7 => regs[dist_reg] = regs[arg0_reg] + regs[arg1_reg],
                8 => regs[dist_reg] = regs[arg0_reg] - regs[arg1_reg],
                9 => regs[dist_reg] = regs[arg0_reg] * regs[arg1_reg],
                10 => regs[dist_reg] = regs[arg0_reg] / regs[arg1_reg],
                11 => {
                    if arg1_reg == 0 {
                        regs[dist_reg] = u64::from_ne_bytes([
                            mem[arg1_reg],
                            mem[arg1_reg + 1],
                            mem[arg1_reg + 2],
                            mem[arg1_reg + 3],
                            mem[arg1_reg + 4],
                            mem[arg1_reg + 5],
                            mem[arg1_reg + 6],
                            mem[arg1_reg + 7],
                        ]);
                    } else {
                        let bytes: [u8; 8] = regs[arg0_reg].to_ne_bytes();
                        mem[arg1_reg] = bytes[0];
                        mem[arg1_reg + 1] = bytes[1];
                        mem[arg1_reg + 2] = bytes[2];
                        mem[arg1_reg + 3] = bytes[3];
                    }
                }
                12 => match regs[REG_ABI::A0 as usize] {
                    1 => exit(0),
                    2 => mem.resize(mem.len() + regs[REG_ABI::A1 as usize] as usize, 0),
                    3 => {
                        mem.pop()
                            .expect("Pop syscall failed on empty memory vector!");
                    }
                    4 => {
                        let len: usize = regs[REG_ABI::A2 as usize] as usize;
                        let start: usize = regs[REG_ABI::A1 as usize] as usize;
                        let mut chars: Vec<char> = Vec::with_capacity(len);
                        for i in start..len {
                            chars[i] = mem[start + i] as char;
                        }
                        let s: String = chars.into_iter().collect();
                        println!("{}", s);
                    }
                    5 => {
                        let len: usize = regs[REG_ABI::A2 as usize] as usize;
                        let start: usize = regs[REG_ABI::A1 as usize] as usize;
                        let stdin = io::stdin();
                        let mut handle = stdin.lock();
                        let mut buffer = String::with_capacity(len);
                        while handle.read_line(&mut buffer).expect("Input error!") > 0 {
                            let mut trimmed: String =
                                String::from(buffer.trim_end_matches(['\r', '\n']));
                            trimmed.truncate(trimmed.floor_char_boundary(len + 1));

                            for i in 0..trimmed.len() {
                                mem[start + i] =
                                    trimmed.chars().nth(i).expect("Char parsing error!") as u8;
                            }
                            buffer.clear();
                            trimmed.clear();
                        }
                    }
                    _ => std::hint::unreachable_unchecked(),
                },
                13 => {
                    if regs[arg0_reg] < regs[arg0_reg] {
                        pc += dist_reg;
                    }
                }
                14 => {
                    if regs[arg0_reg] > regs[arg0_reg] {
                        pc += dist_reg;
                    }
                }
                15 => pc += (regs[dist_reg] as usize) + arg0_reg + arg1_reg,
                _ => std::hint::unreachable_unchecked(),
            }
        }

        pc += 2;
    }
}
