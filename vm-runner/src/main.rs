// Instruction: [bits 4-7 arg1_reg][bits 0-3 arg0_reg], [bits 4-7 dist_reg][bits 0-3 Opcode]
pub enum OP {
    MOV = 0b0000,
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
    POPCNT = 0b1011,
    CLZ = 0b1100,
    BLT = 0b1101,
    BGT = 0b1110,
    BET = 0b1111,
}

fn main() {
    // From compiled bytecode file
    let mem: usize = 16;
    let mut regs: [u64; 16];
    // Reg mapping: x0 = pc; perhaps do this in a enum
}
