pub enum SInstrIdent {
    HLT = 0,
    NOP = 63,
}

pub enum IInstrIdent {
    ADI = 2,
    SBI = 4,
}

pub enum RInstrIdent {
    ADD = 1,
    SUB = 3,
}

#[derive(Clone)]
pub enum RegisterIdent {
    A = 0,
    B = 1,
    C = 2,
    X = 3,
    Y = 4,
    Z = 5,
    R = 6,
    I = 7,
}

pub enum Instr {
    SInstr(SInstrIdent),
    IInstr(IInstrIdent, bool, RegisterIdent, RegisterIdent, u16, u8),
    RInstr(RInstrIdent, bool, RegisterIdent, RegisterIdent, RegisterIdent),
}

fn convert_s_instr(ident: u8) -> Vec<u8> {
    vec![(ident as u8) << 2]
}

// 000000 0 000 000 000
fn convert_r_instr(ident: u16, size: u16, r1: u16, r2: u16, r3: u16) -> Vec<u8> {
    let instr: u16 = ident << 10 | size << 9 | r1 << 6 | r2 << 3 | r3;
    return instr.to_be_bytes().to_vec();
}

// 000000 0 000 000 0000000000000000 000
fn convert_i_instr(ident: u32, size: u32, r1: u32, r2: u32, addr: u32, opt: u32) -> Vec<u8> {
    let instr: u32 = ident << 26 | size << 25 | r1 << 22 | r2 << 19 | addr << 3 | opt;
    return instr.to_be_bytes().to_vec();
}

pub fn convert_instr(i: Vec<Instr>) -> Vec<u8> {
    let mut out = vec![];

    for instr in i {
        match instr {
            Instr::SInstr(ident) => {
                out.append(&mut convert_s_instr(ident as u8));
            }
            Instr::RInstr(ident, size, r1, r2, r3) => {
                out.append(&mut convert_r_instr(ident as u16, size as u16, r1 as u16, r2 as u16, r3 as u16));
            }
            Instr::IInstr(ident, size, r1, r2, addr, opt) => {
                out.append(&mut convert_i_instr(ident as u32, size as u32, r1 as u32, r2 as u32, addr as u32, opt as u32));
            }
        }
    }

    return out;
}