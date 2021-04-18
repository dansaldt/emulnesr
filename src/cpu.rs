
use std::num::Wrapping;

pub struct CPU {
    pub reg_a: u8,
    pub reg_x: u8,
    pub status: u8,
    pub pc: u16,
}

impl CPU {
    pub fn new () -> Self {
        CPU {
            reg_a: 0,
            reg_x: 0,
            status: 0,
            pc: 0,
        }
    }

    fn op_lda(&mut self, opcode: u8, value: u8) {
        // 0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1
        match opcode {
            0xa9 => {
                self.reg_a = value;
                self.flag_update_zero(self.reg_a);
                self.flag_update_negative(self.reg_a);
            },
            0xa5 => todo!(),
            0xb5 => todo!(),
            0xad => todo!(),
            0xbd => todo!(),
            0xb9 => todo!(),
            0xa1 => todo!(),
            0xb1 => todo!(),
            _ => panic!("op_lda invalid opcode {}", opcode)
        }
    }

    fn op_tax(&mut self) {
        self.reg_x = self.reg_a;
        self.flag_update_zero(self.reg_x);
        self.flag_update_negative(self.reg_x);
    }

    fn op_inx(&mut self) {
        self.reg_x = self.reg_x.wrapping_add(1);
        self.flag_update_zero(self.reg_x);
        self.flag_update_negative(self.reg_x);
    }

    fn flag_update_zero(&mut self, result: u8) {
        if result == 0 {
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }
    }

    fn flag_update_negative(&mut self, result: u8) {
        if result & 0b1000_0000 != 0 {
            self.status |= 0b1000_0000;
        } else {
            self.status &= 0b0111_1111;
        }
    }

  fn read_byte (&mut self, program: &Vec<u8>) -> u8 {
        let result = program[self.pc as usize];
        self.pc += 1;
        result
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.pc = 0;

        loop {
            let opcode = self.read_byte(&program);

            match opcode {
                0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1  => {
                    let param = self.read_byte(&program);
                    self.op_lda(opcode, param);
                }
                0xaa => self.op_tax(),
                0xe8 => self.op_inx(),
                0x00 => return, // set break flag omitted as there's no affect
                _ => todo!()
            }
        } // end loop
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.reg_a, 0x05);
        assert_eq!(cpu.reg_a & 0b0000_0010, 0b00);
        assert_eq!(cpu.reg_a & 0b1000_0000, 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag () {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert_eq!(cpu.status & 0b0000_0010, 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.reg_a = 10;
        cpu.interpret(vec![0xaa, 0x00]);
        assert_eq!(cpu.reg_a, cpu.reg_x);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.reg_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.reg_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.reg_x, 1)
    }
}