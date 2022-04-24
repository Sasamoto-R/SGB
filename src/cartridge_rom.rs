use crate::io_controller::IOController;
use std::io::{BufReader, Read, Seek, SeekFrom};

use std::default;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::fs::File;
use anyhow::{bail,Context,Result};

pub struct Cartridge {
    rom: Vec<u8>,
    ram: Vec<u8>,
    mbc_type: u8,
    ram_enable: u8,
    rom_bank_num: u8,
    ram_bank_num: u8,
    bank_mode:    u8,
}

// カートリッジで使用されているMBC(存在する場合)及びカードリッジにさらに外部ハードウェアが存在するかどうか指定
#[derive(FromPrimitive, Debug)]
enum CartridgeType {
   Rom_only                       = 0x00,
   Mbc1                           = 0x01,
   Mbc1_ram                       = 0x02,
   Mbc1_ram_battery               = 0x03,
   Mbc2                           = 0x05,
   Mbc2_battery                   = 0x06,
   Rom_ram                        = 0x08,
   Rom_ram_battery                = 0x09,
   Mmm01                          = 0x0b,
   Mmm01_ram                      = 0x0c,
   Mmm01_ram_battery              = 0x0d,
   Mbc3_timer_battery             = 0x0f,
   Mbc3_timer_ram_battery         = 0x10,
   Mbc3                           = 0x11,
   Mbc3_ram                       = 0x12,
   Mbc3_ram_battery               = 0x13,
   Mbc5                           = 0x19,
   Mbc5_ram                       = 0x1a,
   Mbc5_ram_battery               = 0x1b,
   Mbc5_rumble                    = 0x1c,
   Mbc5_rumble_ram                = 0x1d,
   Mbc5_rumble_ram_battery        = 0x1e,
   Mbc6                           = 0x20,
   Mbc7_sensor_rumble_ram_battery = 0x22,
   Pocket_camera                  = 0xfc,
   Bandai_tama5                   = 0xfd,
   HuC3                           = 0xfe,
   HuC1_ram_battery               = 0xff,
}

impl Cartridge {
    pub fn new(file_name: &str) -> Self {
    }
}
