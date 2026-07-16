macro_rules! define_opcodes {
    ($( $name:ident = $val:expr => $str_name:expr ),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr(u16)]
        pub enum Opcode {
            $( $name = $val, )*
        }

        impl Opcode {
            // This function takes "HALT", "LOAD", etc. and converts it to u64
            pub fn from_str(s: &str) -> Option<u64> {
                match s {
                    $( $str_name => Some(Opcode::$name as u64), )*
                    _ => None,
                }
            }
        }

        impl TryFrom<u16> for Opcode {
            type Error = ();
            fn try_from(v: u16) -> Result<Self, Self::Error> {
                match v {
                    $( $val => Ok(Opcode::$name), )*
                    _ => Err(()),
                }
            }
        }
    };
}

define_opcodes! {
    Halt     = 0 => "HLT",      // Halts execution
    Add      = 2 => "ADD",      // reg1 reg2 reg3 / reg1 = reg2 + reg3

    Jmp      = 4 => "JMP",      // sym / pc = sym
    SaveDisk = 5 => "SDK",      // reg1 reg2 reg3 / drive[reg1] = memory[reg2..reg3]
    Sub      = 6 => "SUB",      // reg1 reg2 reg3 / reg1 = reg2 + reg3
    Mul      = 7 => "MUL",      // reg1 reg2 reg3 / reg1 = reg2 + reg3
    Div      = 8 => "DIV",      // reg1 reg2 reg3 / reg1 = reg2 + reg3
    JmpAbs   = 9 => "JAB",      // imm32 / pc = imm32
    JumpZero = 10 => "JZF",     // sym reg / reg = 0 -> pc = sym
    LoadImm     = 11 => "LDI",     // reg imm / reg = imm

    DTM     = 13 => "DTM",     // reg reg reg / mem start, start sector, sector count
    LD8 = 14 => "LDB", // reg reg / Load byte from memory
    LD16  = 15 => "LDW", // reg reg / Load word from memory
    LD64  = 16 => "LDQ", // reg reg / Load qword from memory
    ST8  = 17 => "STB", // reg reg / mem[addr_reg] = low 8 bits of value_reg
    ST16 = 18 => "STW", // reg reg / mem[addr_reg..addr_reg+2] = low 16 bits
    ST64 = 19 => "STQ", // reg reg / mem[addr_reg..addr_reg+8] = full 64 bits
    JGE = 20 => "JGE", // sym reg1 reg2 / jump if reg1 >= reg2
    SYS = 21 => "SYS",
    JumpEqual = 22 => "JEQ",// sym reg1 reg2 / jump if reg1 = reg2
    WFI = 23 => "WFI", // Wait for Interrupt
    AND = 24 => "AND", // reg1 reg2 reg3 / reg1 = reg2 & reg3
    MOD = 25 => "MOD", //reg1 reg2 reg3 / reg1 = reg2 % reg3
    PUSH = 26 => "PUSH", // reg
    POP = 27 => "POP", // reg
    CALL = 28 => "CLL",
    RET = 29 => "RET",
    OR = 30 => "OR", //reg1 reg2 reg3 / reg 1 = reg2 | reg3
    SHL = 31 => "SHL", // reg1 reg2 reg3 / reg 1 = reg2 << reg3
    SHR = 32 => "SHR", // reg1 reg2 reg3 / reg 1 = reg2 >> reg3
    JLE = 33 => "JLE", // sym reg2 reg3 / jump if reg <= reg2
    JAR = 34 => "JAR", // reg / pc = val at reg
    INC = 35 => "INC", // reg / reg++
    DEC = 36 => "DEC", // reg / reg--
}