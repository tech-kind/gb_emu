pub struct WRam {
    svbk: u8,
    ram: Box<[u8; 0x8000]>,
}

impl WRam {
    pub fn new() -> Self {
        Self {
            svbk: 0,
            ram: Box::new([0; 0x8000]),
        }
    }
    pub fn read(&self, addr: u16) -> u8 {
        if addr == 0xFF70 {
            return self.svbk;
        }
        assert!(addr >= 0xC000 && addr <= 0xFDFF);
        self.ram[(addr as usize) & 0x1fff]
    }
    pub fn write(&mut self, addr: u16, val: u8) {
        if addr == 0xFF70 {
            self.svbk = val;
            return;
        }
        self.ram[(addr as usize) & 0x1fff] = val
    }
}
