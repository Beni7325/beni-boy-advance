pub struct Timer {
    pub div: u16,
    pub tima: u8,
    pub tma: u8,
    pub tac: u8,
    last_and: u8
}

impl Timer {

    pub fn new() -> Timer {
        Timer {
            div: 0x0000,
            tima: 0x00,
            tma: 0x00,
            tac: 0xF8,
            last_and: 0
        }
    }

    pub fn tick(&mut self, m_cycles: u64, interrupt_flag: &mut u8) {

        for _ in 0..m_cycles {

            self.div = self.div.wrapping_add(4);

            let bit = match self.tac & 0x3 {
                0 => 9,
                1 => 3,
                2 => 5,
                3 => 7,
                _ => unreachable!()
            };

            let div_bit = ((self.div >> bit) & 1) as u8;
            let timer_enable = (self.tac & 0x04) >> 2;
            let and_res = div_bit & timer_enable;

            // TIMA increment
            if and_res < self.last_and {

                self.tima = self.tima.wrapping_add(1);

                // Interrupt if it overflows
                if self.tima == 0 {
                    self.tima = self.tma;
                    *interrupt_flag |= 0x04;
                }

            }
            self.last_and = and_res;
        }

    }

}
