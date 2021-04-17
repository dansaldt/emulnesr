
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

    fn op_lda(&mut self, value: u8) {
        self.reg_a = value;
        self.flag_update_zero(self.reg_a);
        self.flag_update_negative(self.reg_a);
    }

    fn op_tax(&mut self) {
        self.reg_x = self.reg_a;
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

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.pc = 0;

        loop {
            let opcode = program[self.pc as usize];
            self.pc += 1;

            match opcode {
                0xa9 => {
                    let param = program[self.pc as usize];
                    self.pc += 1;
                    self.op_lda(param);
                }
                0xaa => self.op_tax(),
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
        assert!(cpu.reg_a & 0b0000_0010 == 0b00);
        assert!(cpu.reg_a & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag () {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.reg_a = 10;
        cpu.interpret(vec![0xaa, 0x00]);
        assert_eq!(cpu.reg_a, cpu.reg_x);
    }


}