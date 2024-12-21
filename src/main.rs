use std::{fmt, fs::File, io::Read, io::Write};

#[derive(Debug)]
enum Opcode {
    Mov
}

#[derive(Debug)]
enum Direction {
    Source, // Instruction source is specificed in the REG field
    Destination, // Instruction destionation is specified in the REG field
}

#[derive(Debug, Clone)]
enum WordOrByte {
    Word,
    Byte
}

struct Mod {
    bits: (bool, bool),
}

impl fmt::Debug for Mod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.bits.0 as u8, self.bits.1 as u8)
    }
}

struct Reg {
    bits: (bool, bool, bool),
}

impl fmt::Debug for Reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.bits.0 as u8, self.bits.1 as u8, self.bits.2 as u8)
    }
}


struct RM {
    bits: (bool, bool, bool),
}

impl fmt::Debug for RM {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.bits.0 as u8, self.bits.1 as u8, self.bits.2 as u8)
    }
}

#[derive(Debug)]
enum Place {
    AL,
    CL,
    DL,
    BL,
    AH,
    CH,
    DH,
    BH,
    
    AX,
    CX,
    DX,
    BX,
    SP,
    BP,
    SI,
    DI,
}

#[derive(Debug)]
struct Instruction {
    mnemonic: Opcode,
    destination: Place,
    source: Place,
}

const FILE_BASE_NAME: &str = "mov_example";

fn main() {
    let input_file = format!("./examples/targets/{}", FILE_BASE_NAME);
    let output_file_name = format!("./examples/{}_inst.asm", FILE_BASE_NAME);

    let mut output_file = File::create(&output_file_name).expect("Failed to create output file");
    writeln!(output_file, "bits 16\n").expect("Failed to write instruction to file");

    let mut file = File::open(&input_file).expect("Failed to open file");
    let mut buffer = Vec::new();
    
    file.read_to_end(&mut buffer).expect("Failed to read file");

    for chunk in buffer.chunks(2) {
        if chunk.len() < 2 {
            break; // If there's an incomplete pair, exit the loop
        }

        let first_byte = chunk[0];
        let second_byte = chunk[1];

        let (opcode, direction, word_or_byte) = decode_first_byte(first_byte);
        let (modd, reg, rm) = decode_second_byte(second_byte);

        let inst = to_asm(opcode, direction, word_or_byte, modd, reg, rm);
        let inst = format!("{:?} {:?}, {:?}", inst.mnemonic, inst.destination, inst.source).to_lowercase();

        writeln!(output_file, "{}", inst).expect("Failed to write instruction to file");
    }

    println!("Successfully wrote instruction to file: {}", output_file_name);
}

fn to_asm(opcode: Opcode, direction: Direction, word_or_byte: WordOrByte, _modd: Mod, reg: Reg, rm: RM) -> Instruction {

    let reg_place = reg_to_place(reg, word_or_byte.clone());
    let rm_place = rm_to_place(rm, word_or_byte);

    let (destination, source) = match direction {
        Direction::Destination => (reg_place, rm_place),
        Direction::Source => (rm_place, reg_place),
    };

    return Instruction {
        mnemonic: opcode,
        destination,
        source,
    }
}

fn rm_to_place(rm: RM, word_or_byte: WordOrByte) -> Place {

    match word_or_byte {
        WordOrByte::Word => {
            match rm.bits {
                (false, false, false) => {
                    // Handle case for (0, 0, 0)
                    return Place::AL
                },
                (false, false, true) => {
                    // Handle case for (0, 0, 1)
                    return Place::CL
                },
                (false, true, false) => {
                    // Handle case for (0, 1, 0)
                    return Place::DL
                },
                (false, true, true) => {
                    // Handle case for (0, 1, 1)
                    return Place::BL
                },
                (true, false, false) => {
                    // Handle case for (1, 0, 0)
                    return Place::AH
                },
                (true, false, true) => {
                    // Handle case for (1, 0, 1)
                    return Place::CH
                },
                (true, true, false) => {
                    // Handle case for (1, 1, 0)
                    return Place::DH
                },
                (true, true, true) => {
                    // Handle case for (1, 1, 1)
                    return Place::BH
                },
            }
        },
        WordOrByte::Byte => {
            match rm.bits {
                (false, false, false) => {
                    // Handle case for (0, 0, 0)
                    return Place::AX
                },
                (false, false, true) => {
                    // Handle case for (0, 0, 1)
                    return Place::CX
                },
                (false, true, false) => {
                    // Handle case for (0, 1, 0)
                    return Place::DX
                },
                (false, true, true) => {
                    // Handle case for (0, 1, 1)
                    return Place::BX
                },
                (true, false, false) => {
                    // Handle case for (1, 0, 0)
                    return Place::SP
                },
                (true, false, true) => {
                    // Handle case for (1, 0, 1)
                    return Place::BP
                },
                (true, true, false) => {
                    // Handle case for (1, 1, 0)
                    return Place::SI
                },
                (true, true, true) => {
                    // Handle case for (1, 1, 1)
                    return Place::DI
                },
            }
        },
    }
}

fn reg_to_place(reg: Reg, word_or_byte: WordOrByte) -> Place {

    match word_or_byte {
        WordOrByte::Word => {
            match reg.bits {
                (false, false, false) => {
                    // Handle case for (0, 0, 0)
                    return Place::AL
                },
                (false, false, true) => {
                    // Handle case for (0, 0, 1)
                    return Place::CL
                },
                (false, true, false) => {
                    // Handle case for (0, 1, 0)
                    return Place::DL
                },
                (false, true, true) => {
                    // Handle case for (0, 1, 1)
                    return Place::BL
                },
                (true, false, false) => {
                    // Handle case for (1, 0, 0)
                    return Place::AH
                },
                (true, false, true) => {
                    // Handle case for (1, 0, 1)
                    return Place::CH
                },
                (true, true, false) => {
                    // Handle case for (1, 1, 0)
                    return Place::DH
                },
                (true, true, true) => {
                    // Handle case for (1, 1, 1)
                    return Place::BH
                },
            }
        },
        WordOrByte::Byte => {
            match reg.bits {
                (false, false, false) => {
                    // Handle case for (0, 0, 0)
                    return Place::AX
                },
                (false, false, true) => {
                    // Handle case for (0, 0, 1)
                    return Place::CX
                },
                (false, true, false) => {
                    // Handle case for (0, 1, 0)
                    return Place::DX
                },
                (false, true, true) => {
                    // Handle case for (0, 1, 1)
                    return Place::BX
                },
                (true, false, false) => {
                    // Handle case for (1, 0, 0)
                    return Place::SP
                },
                (true, false, true) => {
                    // Handle case for (1, 0, 1)
                    return Place::BP
                },
                (true, true, false) => {
                    // Handle case for (1, 1, 0)
                    return Place::SI
                },
                (true, true, true) => {
                    // Handle case for (1, 1, 1)
                    return Place::DI
                },
            }
        },
    }
}

fn decode_second_byte(byte: u8) -> (Mod, Reg, RM) {
    let mut bits = [0u8; 8];
    for i in 0..8 {
        bits[i] = (byte >> (7 - i)) & 1;
    }

    let mod_bits = &bits[0..2];
    let reg_bits = &bits[2..5];
    let rm_bits = &bits[5..8];

    let modd = Mod {
        bits: (mod_bits[0] != 0, mod_bits[1] != 0),
    };

    let reg = Reg {
        bits: (reg_bits[0] != 0, reg_bits[1] != 0, reg_bits[2] != 0),
    };

    let rm = RM {
        bits: (rm_bits[0] != 0, rm_bits[1] != 0, rm_bits[2] != 0),
    };

    (modd, reg, rm)

}

fn decode_first_byte(byte: u8) -> (Opcode, Direction, WordOrByte) {

    let mut bits = [0u8; 8];
    for i in 0..8 {
        bits[i] = (byte >> (7 - i)) & 1;
    }

    let opcode = &bits[0..6];
    let opcode_enum = if opcode == [1, 0, 0, 0, 1, 0] {
        Opcode::Mov
    } else {
        panic!("Unknown opcode")
    };

    let direction = match bits[6] {
        0 => Direction::Source,
        1 => Direction::Destination,
        _ => panic!("direction is not binary")
    };

    let word_or_byte = match bits[7] {
        1 => WordOrByte::Byte,
        0 => WordOrByte::Word,
        _ => panic!("word or byte is not binary")
    };

    return (opcode_enum, direction, word_or_byte)
}