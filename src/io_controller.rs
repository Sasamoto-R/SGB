pub trait IOController {
    fn write(&mut self, address: u16, value: u8);

    fn read(&mut self, address: u16) -> u8;

    fn update(&mut self, tick: u8);
}
