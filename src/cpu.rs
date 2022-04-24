use Bus;

pub struct CPU {
    bus: Bus,
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
    pub new(cartridge_name: &str) -> Self {
        bus:    Bus::new(cartridge_name);
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

    fn read_af_register(&self) -> u16 {
        (self.a as u16) << 8 | self.f as u16
    }

    fn write_af_register(&mut self, u16 value) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0x00FF) as u8;
    }

    fn read_bc_register(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn write_bc_register(&mut self, u16 value) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    fn read_de_register(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    fn write_de_register(&mut self, u16 value) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    fn read_hl_register(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    fn write_hl_register(&mut self, u16 value) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }

    fn read_z_flag(&self) -> bool {
        (self.f >> 7) & 1 == 1
    }

    fn write_z_flag(&mut self, z: bool) {
        self.f = (self.f & !(1 << 7) | (u8::from(z) << 7);
    }

    fn read_n_flag(&self) -> bool {
        (self.f >> 6) & 1 == 1
    }

    fn write_n_flag(&mut self, n: bool) {
        self.f = (self.f & !(1 << 6)) | (u8::from(n) << 6);
    }

    fn read_h_flag(&self) -> bool {
        (self.f >> 5) & 1 == 1
    }

    fn write_h_flag(&mut self, h: bool) {
        self.f = (self.f & !(1 << 5)) | (u8::from(h) << 5);
    }

    fn read_c_flag(&self) -> bool {
        (self.f >> 4) & 1 == 1
    }

    fn write_c_flag(&mut self, c: bool) {
        self.f = (self.f & !(1 << 4)) | (u8::from(c) << 4);
    }



}

