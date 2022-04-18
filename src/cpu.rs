pub struct CPU {
    a:  u8,
    b:  u8,
    c:  u8,
    d:  u8,
    e:  u8,
    h:  u8,
    l:  u8,
    sp: u16,      // Stack Pointer
    pc: u16,      // Program Counter/Pointer
    ime: bool,    // Interrupt Master Emutable
    tick: u8,     // T-Cycle(4.1.94304 MHz)
    halted: bool, // Stop
}

impl CPU {
    pub new() -> Self {
        a:      0,
        b:      0,
        c:      0,
        d:      0,
        e:      0,
        h:      0,
        l:      0,
        sp:     0,
        pc:     0,
        tick:   0,
        halted: false,
        ime:    false,
    }

    pub fn read_af_register(&self) -> u16 {
        (self.a as u16) << 8 | self.f as u16
    }

    pub fn write_af_register(&mut self, u16 value) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0x00FF) as u8;
    }

    pub fn read_bc_register(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn write_bc_register(&mut self, u16 value) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    pub fn read_de_register(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn write_de_register(&mut self, u16 value) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    pub fn read_hl_register(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn write_hl_register(&mut self, u16 value) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }
}

