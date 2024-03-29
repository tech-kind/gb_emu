#[derive(Clone)]
pub struct Bootrom {
    rom: Box<[u8]>,
    active: bool,
}

impl Bootrom {
    pub fn new() -> Self {
        let rom = vec![
            0x31, 0xfe, 0xff, 0x21, 0x00, 0x80, 0x22, 0xcb, 0x6c, 0x28, 0xfb, 0x3e, 0x80, 0xe0,
            0x26, 0xe0, 0x11, 0x3e, 0xf3, 0xe0, 0x12, 0xe0, 0x25, 0x3e, 0x77, 0xe0, 0x24, 0x3e,
            0x54, 0xe0, 0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1a, 0x47, 0xcd, 0xa2, 0x00,
            0xcd, 0xa2, 0x00, 0x13, 0x7b, 0xee, 0x34, 0x20, 0xf2, 0x11, 0xd1, 0x00, 0x0e, 0x08,
            0x1a, 0x13, 0x22, 0x23, 0x0d, 0x20, 0xf9, 0x3e, 0x19, 0xea, 0x10, 0x99, 0x21, 0x2f,
            0x99, 0x0e, 0x0c, 0x3d, 0x28, 0x08, 0x32, 0x0d, 0x20, 0xf9, 0x2e, 0x0f, 0x18, 0xf5,
            0x3e, 0x1e, 0xe0, 0x42, 0x3e, 0x91, 0xe0, 0x40, 0x16, 0x89, 0x0e, 0x0f, 0xcd, 0xb7,
            0x00, 0x7a, 0xcb, 0x2f, 0xcb, 0x2f, 0xe0, 0x42, 0x7a, 0x81, 0x57, 0x79, 0xfe, 0x08,
            0x20, 0x04, 0x3e, 0xa8, 0xe0, 0x47, 0x0d, 0x20, 0xe7, 0x3e, 0xfc, 0xe0, 0x47, 0x3e,
            0x83, 0xcd, 0xca, 0x00, 0x06, 0x05, 0xcd, 0xc3, 0x00, 0x3e, 0xc1, 0xcd, 0xca, 0x00,
            0x06, 0x3c, 0xcd, 0xc3, 0x00, 0x21, 0xb0, 0x01, 0xe5, 0xf1, 0x21, 0x4d, 0x01, 0x01,
            0x13, 0x00, 0x11, 0xd8, 0x00, 0xc3, 0xfe, 0x00, 0x3e, 0x04, 0x0e, 0x00, 0xcb, 0x20,
            0xf5, 0xcb, 0x11, 0xf1, 0xcb, 0x11, 0x3d, 0x20, 0xf5, 0x79, 0x22, 0x23, 0x22, 0x23,
            0xc9, 0xe5, 0x21, 0x0f, 0xff, 0xcb, 0x86, 0xcb, 0x46, 0x28, 0xfc, 0xe1, 0xc9, 0xcd,
            0xb7, 0x00, 0x05, 0x20, 0xfa, 0xc9, 0xe0, 0x13, 0x3e, 0x87, 0xe0, 0x14, 0xc9, 0x3c,
            0x42, 0xb9, 0xa5, 0xb9, 0xa5, 0x42, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0xe0, 0x50,
        ]
        .into();
        Self { rom, active: true }
    }
    pub fn from_data(rom: Box<[u8]>) -> Self {
        Self { rom, active: true }
    }
    pub fn is_active(&self) -> bool {
        self.active
    }
    pub fn read(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }
    pub fn write(&mut self, _: u16, val: u8) {
        self.active &= val == 0;
    }
}
