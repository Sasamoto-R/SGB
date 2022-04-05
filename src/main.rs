mod cartridge;

use pixels::{Pixels, SurfaceTexture};
use std::env;
use std::fs::File;
use std::io::BufReader;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

fn main() {
    let event_loop = EventLoop::new();
    let mut input  = WinitInputHelper::new();

    let size       = LogicalSize::new(160, 144);
    let window     = WindowBuilder::new()
        .with_title("SGB")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(&event_loop)
        .unwrap();

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels  = Pixels::new(160, 144, surface_texture).unwrap();
    
    let args = env::args().collect::<Vec<String>>();

    let mut cartridgeFile = BufReader::new(File::open(args[1].clone()).unwrap());
    let cartridgeInfo = cartridge::CartridgeHeader::new(&mut cartridgeFile).unwrap();
}
