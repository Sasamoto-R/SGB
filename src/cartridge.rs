use std::io::{BufReader, Read, Seek, SeekFrom};

use std::default;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::fs::File;
use anyhow::{bail,Context,Result};

// カートリッジで使用されているMBC(存在する場合)及びカードリッジにさらに外部ハードウェアが存在するかどうか指定
#[derive(FromPrimitive, Debug)]
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

impl Default for CartridgeType {
    fn default() -> Self {
        CartridgeType::Rom_only
    }
}

pub struct CartridgeHeader {
    pub entry_point:             [u8; 0x0004],
    pub logo:                    [u8; 0x0030],
    pub title:                   [u8; 0x0010],
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
}

impl Default for CartridgeHeader {
    fn default() -> Self {
        CartridgeHeader {
            entry_point: Default::default(),
            logo: [0; 0x0030],
            title: Default::default(),
            new_licensee_code: Default::default(),
            sgb_flag: Default::default(),
            cartridge_type: Default::default(),
            rom_size: Default::default(),
            ram_size: Default::default(),
            destination_code: Default::default(),
            old_licensee_code: Default::default(),
            mask_rom_version_number: Default::default(),
            header_check_sum: Default::default(),
            global_check_sum: Default::default(),
        }
    }
}

impl CartridgeHeader {
    pub fn new(reader: &mut BufReader<File>) -> Result<CartridgeHeader> {
        // defaultメソッドで初期化
        let mut cartridgeHeader = CartridgeHeader::default();

        // seekメソッドを使い、reader(読み取ったFileの中身)内の0x0100をスタート位置としてカーソルを移動させる
        reader.seek(SeekFrom::Start(0x0100))?;

        // 0100-0103 - Entry Point 
        reader.read_exact(&mut cartridgeHeader.entry_point[..])?;

        // 0104-0133 - Nintendo Logo
        reader.read_exact(&mut cartridgeHeader.logo[..])?;

        // 0134-0143 - Title
        reader.read_exact(&mut cartridgeHeader.title[..])?;

        // 0144-0145 - New Licensee Code
        reader.read_exact(&mut cartridgeHeader.new_licensee_code[..])?;

        // 0146 - SGB Flag
        cartridgeHeader.sgb_flag = match reader.take(1).bytes().next() {
            Some(Ok(0x00))    => false,
            Some(Ok(0x03))    => true,
            Some(Ok(unknown)) => {
                eprintln!("unknown SGB Flag {:#X}", unknown);
                false
            }
            Some(Err(e))      => bail!("Error occured while reading the SGB Flag {}", e),
            None              => bail!("unexpected EOF while reading the SGB Flag"),
        };

        // 0147 - Cartridge Type
        if let Some(Ok(typ)) = reader.take(1).bytes().next() {
            cartridgeHeader.cartridge_type = FromPrimitive::from_u8(typ).context("unknown mbc type")?;
        } else {
            bail!("failed to parse the Cardridge Type");
        }

        // 0148 - ROM Size
        cartridgeHeader.rom_size = match reader.take(1).bytes().next() {
            Some(Ok(0x00))    => 32 * 1024 as usize,
            Some(Ok(0x01))    => 64 * 1024 as usize,
            Some(Ok(0x02))    => 128 * 1024 as usize,
            Some(Ok(0x03))    => 256 * 1024 as usize,
            Some(Ok(0x04))    => 512 * 1024 as usize,
            Some(Ok(0x05))    => 1 * 1024 * 1024 as usize,
            Some(Ok(0x06))    => 2 * 1024 * 1024 as usize,
            Some(Ok(0x07))    => 4 * 1024 * 1024 as usize,
            Some(Ok(0x08))    => 8 * 1024 * 1024 as usize,
            Some(Ok(0x52))    => (1.1 * 1024.0 * 1024.0) as usize,
            Some(Ok(0x53))    => (1.2 * 1024.0 * 1024.0) as usize,
            Some(Ok(0x54))    => (1.5 * 1024.0 * 1024.0) as usize,
            Some(Ok(unknown)) => {
                eprintln!("unknown ROM Size {:#X}", unknown);
                0
            }
            Some(Err(e))      => bail!("error occurd while reading the ROM Size {}", e),
            None              => bail!("unexpected EOF while reading the ROM Size"),
        };

        // 0149 - RAM Size
        cartridgeHeader.ram_size = match reader.take(1).bytes().next() {
            Some(Ok(0x00))       => 0 as usize,
            Some(Ok(0x01))       => 2 * 1024 * 1024 as usize,
            Some(Ok(0x02))       => 8 * 1024 * 1024 as usize,
            Some(Ok(0x03))       => 32 * 1024 * 1024 as usize,
            Some(Ok(0x04))       => 128 * 1024 * 1024 as usize,
            Some(Ok(0x05))       => 64 * 1024 * 1024 as usize,
            Some(Ok(unknown))    => {
                eprintln!("unknown RAM Size {:#X}", unknown);
                0
            }
            Some(Err(e))         => bail!("error occurd while reading the RAM Size {}", e),
            None                 => bail!("unexpected EOF while reading the RAM Size"),
        };

        // 014A - Destination Code
        cartridgeHeader.destination_code = match reader.take(1).bytes().next() {
            Some(Ok(0x00)) => 0x00,
            Some(Ok(0x01)) => 0x01,
            _              => 0xFF,
        };

        // 014B - Old Licensee Code
        cartridgeHeader.old_licensee_code = reader.take(1).bytes().next()
            .context("failed to parse the Old Licensee Code")??;

        // 014C - Mask ROM Version number
        cartridgeHeader.mask_rom_version_number = reader.take(1).bytes().next()
            .context("failed to parse the Mask ROM Version number")??; 

        // 014D - Header CheckSum
        cartridgeHeader.header_check_sum = reader.take(1).bytes().next().context("failed to parse the Header Checksum")??;

        // 014E-014F - Global Checksum
        reader.read_exact(&mut cartridgeHeader.global_check_sum[..])?;

        Ok(cartridgeHeader)
    }
}
