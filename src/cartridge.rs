use std::io::BufReader;

// カートリッジで使用されているMBC(存在する場合)及びカードリッジにさらに外部ハードウェアが存在するかどうか指定
pub enum CartridgeType {
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

pub struct CartridgeHeader {
    pub entry_point:             [u8; 0x0004],
    pub logo:                    [u8; 0x0030],
    pub title:                   [u8; 0x0010],
    pub maker_code:              [u8; 0x0004],
    pub cgb_flag:                bool,
    pub new_licensee_code:       [u8; 0x0002],
    pub sgb_flag:                bool,
    pub cartridge_type:          CartridgeType, 
    pub rom_size:                usize,
    pub ram_size:                usize,
    pub destination_code:        u8,
    pub old_licensee_code:       u8,
    pub mask_rom_version_number: u8,
    pub header_check_sum:        u8,
    pub global_check_sum:        [u8; 0x0002],
    pub data: Vec<u8>,
}

//    impl CartridgeHeader {
//        pub fn new(reader: &mut BufReader<File>) -> Result<CartridgeHeader> {
//
//        }
//    }
