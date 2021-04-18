
pub struct CPU {
    pub reg_a: u8,
    pub reg_x: u8,
    pub reg_y: u8,
    pub status: u8,
    pub pc: u16,
    pub memory: [u8; 0xffff],
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingModes {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

impl CPU {
    pub fn new () -> Self {
        CPU {
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            status: 0,
            pc: 0,
            memory: [0; 0xffff],
        }
    }

    pub fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn mem_read_u16(&mut self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    pub fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos+1, hi);
    }

    pub fn reset(&mut self) {
        self.reg_a = 0;
        self.reg_x = 0;
        self.status = 0;

        self.pc = self.mem_read_u16(0xfffc);
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xfffc, 0x8000);
    }

    pub fn run(&mut self) {
        // note: we move initialization of program counter from here to load function
        loop {
            let opcode = self.prog_read();

            match opcode {
                0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1  => {
                    let param = self.prog_read();
                    self.op_lda(opcode, param);
                }
                0xaa => self.op_tax(),
                0xe8 => self.op_inx(),
                0x00 => return, // set break flag omitted as there's no affect
                _ => todo!()
            }
        } // end loop
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

    fn prog_read (&mut self) -> u8 {
        let result = self.mem_read(self.pc);
        self.pc += 1;
        result
    }

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.reg_a, 0x05);
        assert_eq!(cpu.reg_a & 0b0000_0010, 0b00);
        assert_eq!(cpu.reg_a & 0b1000_0000, 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag () {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert_eq!(cpu.status & 0b0000_0010, 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.reg_a = 10;
        cpu.load_and_run(vec![0xaa, 0x00]);
        assert_eq!(cpu.reg_a, cpu.reg_x);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.reg_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xe8, 0x00]);

        assert_eq!(cpu.reg_x, 1)
    }
}