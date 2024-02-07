use std::{env, fs::File, io::Read};

pub const CPU_CLOCK_HZ: u128 = 4_194_304;

pub const SAMPLES: usize = 512;
pub const SAMPLE_RATE: u128 = 48000;

pub const LCD_WIDTH: usize = 160;
pub const LCD_HEIGHT: usize = 144;
pub const LCD_PIXELS: usize = LCD_WIDTH * LCD_HEIGHT;

mod apu;
mod audio;
pub mod bootrom;
pub mod cartridge;
pub mod cpu;
mod gameboy;
mod hram;
pub mod joypad;
mod lcd;
pub mod peripherals;
mod ppu;
mod timer;
mod wram;

fn file2vec(fname: &String) -> Vec<u8> {
    if let Ok(mut file) = File::open(fname) {
        let mut ret = vec![];
        file.read_to_end(&mut ret).unwrap();
        ret
    } else {
        panic!("Cannot open {}.", fname);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let cartridge_raw = file2vec(&args[1]);
    let save = if args.len() >= 3 {
        Some(file2vec(&args[2]))
    } else {
        None
    };

    let cartridge = cartridge::Cartridge::new(cartridge_raw.into(), save);
    let bootrom = bootrom::Bootrom::new();

    let mut gameboy = gameboy::GameBoy::new(bootrom, cartridge);
    gameboy.run();
}
