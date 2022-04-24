use Cartridge_rom;

pub struct Bus {
    cartridge_rom: Cartridge_rom,
    ram: u8,
    hram: u8,
    pub joypad: Joypad,
    timer: Timer,
    pub ppu: PPU,
    pub int_flag: u8,
    pub int_enable: u8,
}
