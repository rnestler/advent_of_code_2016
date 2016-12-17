use std::str::FromStr;

pub fn puzzle(input: &str) -> i32 {
    let code: Vec<_> = input.lines().enumerate().map(|(k,v)| {
        Instruction::from_str(v).expect(&format!("Failed to parse line {}: {}", k, v))
    }).collect();
    let mut machine = Machine::new(code);
    while let Some(()) = machine.execute() {
        println!("{:?}", machine);
    }
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
    Jnz(FromLocation, i32),
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

    pub fn execute(&mut self) -> Option<()> {
        let inst = self.code[self.pc].clone();
        match inst {
            Instruction::Dec(reg) => self.decrement(reg),
            Instruction::Inc(reg) => self.increment(reg),
            Instruction::Cpy(src, dst) => self.copy(src, dst),
            Instruction::Jnz(value, offset) => self.jnz(value, offset),
        }
        self.pc += 1;
        if self.pc >= self.code.len() {
            None
        } else {
            Some(())
        }
    }

    pub fn get_reg(&mut self, reg: Register) -> i32 {
        match reg {
            Register::A => self.reg_a,
            Register::B => self.reg_b,
            Register::C => self.reg_c,
            Register::D => self.reg_d,
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
        }
    }

    pub fn copy(&mut self, src: FromLocation, dst: Register) {
        let src = match src {
            FromLocation::Int(i) => i,
            FromLocation::Reg(Register::A) => self.reg_a,
            FromLocation::Reg(Register::B) => self.reg_b,
            FromLocation::Reg(Register::C) => self.reg_c,
            FromLocation::Reg(Register::D) => self.reg_d,
        };
        match dst {
            Register::A => self.reg_a = src,
            Register::B => self.reg_b = src,
            Register::C => self.reg_c = src,
            Register::D => self.reg_d = src,
        }
    }

    pub fn decrement(&mut self, reg: Register) {
        match reg {
            Register::A => self.reg_a -= 1,
            Register::B => self.reg_b -= 1,
            Register::C => self.reg_c -= 1,
            Register::D => self.reg_d -= 1,
        }
    }

    pub fn increment(&mut self, reg: Register) {
        match reg {
            Register::A => self.reg_a += 1,
            Register::B => self.reg_b += 1,
            Register::C => self.reg_c += 1,
            Register::D => self.reg_d += 1,
        }
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
        assert_eq!(instruction, Instruction::Jnz(FromLocation::Reg(Register::A), 2));
    }

    #[test]
    fn parse_jnz_2() {
        let instruction: Instruction = "jnz 1 5".parse().unwrap();
        assert_eq!(instruction, Instruction::Jnz(FromLocation::Int(1), 5));
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
        assert_eq!(puzzle(input), 42);
    }
}
