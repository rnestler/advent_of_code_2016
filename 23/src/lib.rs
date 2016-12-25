use std::str::FromStr;

pub fn puzzle(input: &str, reg_a_start_value: i32) -> i32 {
    let code: Vec<_> = input.lines().enumerate().map(|(k,v)| {
        Instruction::from_str(v).expect(&format!("Failed to parse line {}: {}", k, v))
    }).collect();
    let mut machine = Machine::new(code);
    *machine.get_reg_mut(Register::A) = reg_a_start_value;
    machine.run();
    machine.get_reg(Register::A)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Register {
    A,
    B,
    C,
    D,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FromLocation {
    Int(i32),
    Reg(Register),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Instruction {
    Inc(Register),
    Dec(Register),
    Cpy(FromLocation, Register),
    Jnz(FromLocation, FromLocation),
    Tgl(FromLocation),
    Invalid,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        match parts.next() {
            Some("cpy") => Ok(Instruction::Cpy(
                    parts.next().ok_or("err")?.parse()?,
                    parts.next().ok_or("err")?.parse()?)
                ),
            Some("inc") => Ok(Instruction::Inc(
                    parts.next().ok_or("err")?.parse()?)
                ),
            Some("dec") => Ok(Instruction::Dec(
                    parts.next().ok_or("err")?.parse()?)
                ),
            Some("jnz") => Ok(Instruction::Jnz(
                    parts.next().ok_or("err")?.parse()?,
                    parts.next().ok_or("err")?.parse().unwrap()
                    )
                ),
            Some("tgl") => Ok(Instruction::Tgl(
                    parts.next().ok_or("err")?.parse()?,
                    )
                ),

            _ => Err("Not implemented".into()),
        }
    }
}

impl FromStr for FromLocation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(FromLocation::Reg(Register::A)),
            "b" => Ok(FromLocation::Reg(Register::B)),
            "c" => Ok(FromLocation::Reg(Register::C)),
            "d" => Ok(FromLocation::Reg(Register::D)),
            s => Ok(FromLocation::Int(s.parse().unwrap())),
        }
    }
}

impl FromStr for Register {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            "c" => Ok(Register::C),
            "d" => Ok(Register::D),
            _ => Err(format!("Invalid Register {}", s).into()),
        }
    }
}


#[derive(Debug, Default)]
pub struct Machine {
    reg_a: i32,
    reg_b: i32,
    reg_c: i32,
    reg_d: i32,
    pc: usize,
    code: Vec<Instruction>,
}

impl Machine {

    pub fn new(code: Vec<Instruction>) -> Machine {
        Machine { code: code, ..Default::default() }
    }

    pub fn run(&mut self) {
        while let Some(()) = self.execute() {
        }
    }

    pub fn execute(&mut self) -> Option<()> {
        let inst = self.code[self.pc].clone();
        match inst {
            Instruction::Dec(reg) => self.decrement(reg),
            Instruction::Inc(reg) => self.increment(reg),
            Instruction::Cpy(src, dst) => self.copy(src, dst),
            Instruction::Jnz(value, FromLocation::Int(offset)) => self.jnz(value, offset),
            Instruction::Jnz(value, FromLocation::Reg(reg)) => {
                let offset = self.get_reg(reg);
                self.jnz(value, offset)
            }
            Instruction::Invalid => (),
            Instruction::Tgl(FromLocation::Int(offset)) => self.toggle(offset),
            Instruction::Tgl(FromLocation::Reg(reg)) => {
                let reg = self.get_reg(reg);
                self.toggle(reg)
            }
        }
        self.pc += 1;
        if self.pc >= self.code.len() {
            None
        } else {
            Some(())
        }
    }

    pub fn toggle(&mut self, offset: i32) {
        let position = if offset > 0 {
            self.pc + offset as usize
        } else {
            self.pc - (-offset) as usize
        };
        if position >= self.code.len() {
            return
        }
        self.code[position] = Self::toggle_instruction(self.code[position].clone());
    }

    pub fn toggle_instruction(instruction: Instruction) -> Instruction {
        match instruction {
            Instruction::Inc(reg) => Instruction::Dec(reg),
            Instruction::Dec(reg) | Instruction::Tgl(FromLocation::Reg(reg)) => Instruction::Inc(reg),
            //Instruction::Tgl(FromLocation::Int(_)) => Instruction::Invalid,

            //Instruction::Jnz(FromLocation::Int(_), FromLocation::Int(_)) => Instruction::Invalid,
            Instruction::Jnz(from, FromLocation::Reg(reg)) => Instruction::Cpy(from, reg),

            Instruction::Cpy(from, reg) => Instruction::Jnz(from, FromLocation::Reg(reg)),
            //Instruction::Invalid => Instruction::Invalid,
            _ => Instruction::Invalid,
        }
    }

    pub fn get_reg(&self, reg: Register) -> i32 {
        match reg {
            Register::A => self.reg_a,
            Register::B => self.reg_b,
            Register::C => self.reg_c,
            Register::D => self.reg_d,
        }
    }

    pub fn get_reg_mut(&mut self, reg: Register) -> &mut i32 {
        match reg {
            Register::A => &mut self.reg_a,
            Register::B => &mut self.reg_b,
            Register::C => &mut self.reg_c,
            Register::D => &mut self.reg_d,
        }
    }

    pub fn jnz(&mut self, value: FromLocation, offset: i32) {
        let value = match value {
            FromLocation::Reg(reg) => self.get_reg(reg),
            FromLocation::Int(val) => val,
        };
        if value != 0 {
            if offset > 0 {
                self.pc += offset as usize;
            } else {
                self.pc -= (-offset) as usize;
            }
            // pc will get autoincremented afterwards
            self.pc -= 1;
        }
    }

    pub fn copy(&mut self, src: FromLocation, dst: Register) {
        let src = match src {
            FromLocation::Int(i) => i,
            FromLocation::Reg(reg) => self.get_reg(reg),
        };
        *self.get_reg_mut(dst) = src;
    }

    pub fn decrement(&mut self, reg: Register) {
        *self.get_reg_mut(reg) -= 1;
    }

    pub fn increment(&mut self, reg: Register) {
        *self.get_reg_mut(reg) += 1;
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_cpy() {
        let instruction: Instruction = "cpy 41 a".parse().unwrap();
        assert_eq!(instruction, Instruction::Cpy(FromLocation::Int(41), Register::A));
    }

    #[test]
    fn parse_inc() {
        let instruction: Instruction = "inc a".parse().unwrap();
        assert_eq!(instruction, Instruction::Inc(Register::A));
    }

    #[test]
    fn parse_dec() {
        let instruction: Instruction = "dec a".parse().unwrap();
        assert_eq!(instruction, Instruction::Dec(Register::A));
    }

    #[test]
    fn parse_jnz() {
        let instruction: Instruction = "jnz a 2".parse().unwrap();
        assert_eq!(instruction, Instruction::Jnz(FromLocation::Reg(Register::A), FromLocation::Int(2)));
    }

    #[test]
    fn parse_jnz_2() {
        let instruction: Instruction = "jnz 1 5".parse().unwrap();
        assert_eq!(instruction, Instruction::Jnz(FromLocation::Int(1), FromLocation::Int(5)));
    }

    #[test]
    fn sample() {
        let input =
"cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";
        assert_eq!(puzzle(input, 0), 42);
    }

    #[test]
    fn test_toggle_instruction_inc() {
        let instruction = Instruction::Inc(Register::A);
        assert_eq!(Machine::toggle_instruction(instruction), Instruction::Dec(Register::A));
    }

    #[test]
    fn test_toggle_instruction_dec() {
        let instruction = Instruction::Dec(Register::A);
        assert_eq!(Machine::toggle_instruction(instruction), Instruction::Inc(Register::A));
    }

    #[test]
    fn test_sample_input() {
        let input =
"cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";
        assert_eq!(puzzle(input, 0), 3);
    }
}
