use crate::{error::AssemblerError, instruction::{IInstrIdent, Instr, RInstrIdent, RegisterIdent, SInstrIdent}};
use once_cell::sync::Lazy;
use regex::Regex;

enum LineType {
    SInstr,
    IInstr,
    RInstr,
    Value,
    Label,
    Empty,
}

fn trim_zeros(v: Vec<u8>) -> Vec<u8> {
    let mut out = vec![];
    let mut trailing = true;
    for i in v {
        if trailing && i != 0 {
            trailing = false;
            out.push(i);
        } else if !trailing {
            out.push(i)
        }
    }
    return out;
}

fn register_name_to_ident(reg: char) -> Result<RegisterIdent, AssemblerError> {
    return match reg {
        'a' => Ok(RegisterIdent::A),
        'b' => Ok(RegisterIdent::B),
        'c' => Ok(RegisterIdent::C),
        'x' => Ok(RegisterIdent::X),
        'y' => Ok(RegisterIdent::Y),
        'z' => Ok(RegisterIdent::Z),
        'i' => Ok(RegisterIdent::I),
        'r' => Ok(RegisterIdent::R),
        _ => Err(AssemblerError::InvalidRegisterError)
    }
}

fn register_scalar_to_ident(scalar: String) -> Result<(RegisterIdent, u8), AssemblerError> {
    let scalar_char = scalar.chars().nth(0).unwrap();
    let mut scalar_int: u8 = 1;
    match scalar_char {
        '2' => { scalar_int = 2; },
        '4' => { scalar_int = 4; },
        _ => return Err(AssemblerError::InvalidScalarError)
    };

    let register_name = scalar.chars().nth(3).unwrap();
    return match register_name_to_ident(register_name) {
        Ok(reg) => Ok((reg, scalar_int)),
        Err(err) => Err(err)
    };
}

fn i_instr_name_to_ident(instr: String) -> Result<IInstrIdent, AssemblerError> {
    return match instr.to_lowercase().as_str() {
        "add" => Ok(IInstrIdent::ADD),
        "sub" => Ok(IInstrIdent::SUB),
        _ => Err(AssemblerError::InvalidInstructionError)
    }
}

fn r_instr_name_to_ident(instr: String) -> Result<RInstrIdent, AssemblerError> {
    return match instr.to_lowercase().as_str() {
        "add" => Ok(RInstrIdent::ADD),
        "sub" => Ok(RInstrIdent::SUB),
        _ => Err(AssemblerError::InvalidInstructionError)
    }
}

fn s_instr_name_to_ident(instr: String) -> Result<SInstrIdent, AssemblerError> {
    return match instr.to_lowercase().as_str() {
        "hlt" => Ok(SInstrIdent::HLT),
        "nop" => Ok(SInstrIdent::NOP),
        _ => Err(AssemblerError::InvalidInstructionError)
    }
}

fn handle_regex_line(line: String, regex: Regex, t: LineType) -> Result<(Vec<String>, LineType), AssemblerError>{
    let mut mat = regex.find_iter(&line);
    if let Some(m) = mat.next() {
        if m.start() != 0 {
            return Err(AssemblerError::InvalidSyntaxError);
        }

        let trail = line[m.end()..].trim();

        if !trail.starts_with(";") && !trail.is_empty() {
            return Err(AssemblerError::InvalidSyntaxError)
        }

        let captures = regex.captures_iter(&line).next().unwrap();
        let mut tokens = vec![];

        for cap in captures.iter() {
            if let Some(c) = cap {
                tokens.push(c.as_str().to_owned());
            }
        }

        return Ok((tokens[1..].to_vec(), t));
    }
    
    return Err(AssemblerError::InvalidSyntaxError);
}

fn parse_value(input: String) -> Result<Vec<u8>, AssemblerError> {
    if input.starts_with("0x") {
        if let Ok(numb) = usize::from_str_radix(input.trim_start_matches("0x"), 16) {
            return Ok(trim_zeros(numb.to_be_bytes().to_vec()));
        } else {
            return Err(AssemblerError::LiteralOverflowError);
        }
    } else if input.starts_with("0b") {
        if let Ok(numb) = usize::from_str_radix(input.trim_start_matches("0b"), 2) {
            return Ok(trim_zeros(numb.to_be_bytes().to_vec()));
        } else {
            return Err(AssemblerError::LiteralOverflowError);
        }
    } else if input.starts_with("\"") {
        if let Ok(s) = snailquote::unescape(&input) {
            return Ok(s.into_bytes().to_vec());
        } else {
            return Err(AssemblerError::InvalidStringError)
        }
    } else {
        if let Ok(numb) = usize::from_str_radix(&input, 10) {
            return Ok(trim_zeros(numb.to_be_bytes().to_vec()));
        } else {
            return Err(AssemblerError::LiteralOverflowError);
        }
    }
}

fn parse_line(input: String) -> Result<(Vec<String>, LineType), AssemblerError> {
    let trimmed = input.trim();
    if trimmed.is_empty() || trimmed.starts_with(";") {
        return Ok((vec![], LineType::Empty));
    }

    static R_INSTR_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"([a-zA-Z]{3})(b?) +(%[abcrixyz]) *, *(%[abcrixyz])(?: *, *(%[abcrixyz]))?")
            .unwrap()
    });
    static I_INSTR_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"([a-zA-Z]{3})(b?) +(%[abcrixyz]) *, *(?:([24]\(%[abcrixyz]\)|%[abcrixyz]) *, *)?(\$?(?:0x[a-fA-F0-9]+|0b[01]+|[0-9]+))").unwrap()
    });
    static S_INSTR_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"([a-zA-Z]{3})").unwrap());
    static VAL_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"#value +("(?:[^"\\]|\\.)*"|0x[0-9a-fA-F]+|0b[01]+|[0-9]+)"#).unwrap());

    if VAL_RE.is_match(trimmed) {
        return handle_regex_line(trimmed.to_owned(), VAL_RE.clone(), LineType::Value);
    } else if R_INSTR_RE.is_match(trimmed) {
        return handle_regex_line(trimmed.to_owned(), R_INSTR_RE.clone(), LineType::RInstr);
    } else if I_INSTR_RE.is_match(trimmed) {
        return handle_regex_line(trimmed.to_owned(), I_INSTR_RE.clone(), LineType::IInstr);
    } else if S_INSTR_RE.is_match(trimmed) {
        return handle_regex_line(trimmed.to_owned(), S_INSTR_RE.clone(), LineType::SInstr);
    } 
    
    return Err(AssemblerError::InvalidSyntaxError)
}

fn line_to_instr(input: Vec<String>, line_type: LineType) -> Result<Option<Instr>, AssemblerError> {
    match line_type {
        LineType::IInstr => return Ok(None),
        LineType::SInstr => return Ok(Some(Instr::SInstr(s_instr_name_to_ident(input[0].clone())?))),
        LineType::RInstr => {
            let ident = r_instr_name_to_ident(input[0].clone())?;
            let size = input[1].len() == 0;
            let reg1 = register_name_to_ident(input[2].chars().nth(1).unwrap())?;
            let reg2 = register_name_to_ident(input[3].chars().nth(1).unwrap())?;
            let reg3 = if input.len() == 4 {
                reg1.clone()
            } else {
                register_name_to_ident(input[4].chars().nth(1).unwrap())?
            };
            return Ok(Some(Instr::RInstr(ident, size, reg1, reg2, reg3)));
        },
        LineType::Value => {
            let value = parse_value(input[0].clone())?;
            return Ok(Some(Instr::Value(value)))
        }
        _ => return Ok(None),
    }
}

pub fn parse(input: String) -> Result<Vec<Instr>, AssemblerError> {
    let mut tokens: Vec<(Vec<String>, LineType)> = vec![];

    for line in input.split("\n") {
        let parsed = parse_line(line.to_owned());
        if let Ok(t) = parsed {
            tokens.push(t);
        } else if let Err(error) = parsed {
            return Err(error);
        }
    }

    let mut instrs: Vec<Instr> = vec![];

    for (line, line_type) in tokens {
        println!("{:?}", line.clone());
        let parsed = line_to_instr(line, line_type);
        if let Ok(pot_instr) = parsed {
            // TODO: Handle Labels
            if let Some(instr) = pot_instr {
                instrs.push(instr);
            }
        } else if let Err(error) = parsed {
            return Err(error);
        }
    }

    return Ok(instrs);
}
