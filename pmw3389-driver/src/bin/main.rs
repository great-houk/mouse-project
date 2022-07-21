#![feature(slice_as_chunks)]

fn main() {
    gen_rom();
}

fn gen_rom() {
    const FILE: &[u8; 16376] = include_bytes!("../../rom_ascii.txt");
    const LINE_SIZE: usize = 4;
    let (chars, _): (&[[u8; LINE_SIZE]], _) = FILE.as_chunks();
    let mut bytes = [0; FILE.len() / LINE_SIZE];
    for (i, line) in chars.iter().enumerate() {
        let hex = std::str::from_utf8(&line[0..2]).unwrap();
        let val = u8::from_str_radix(hex, 16).unwrap();
        bytes[i] = val;
    }
    std::fs::write("rom.hex", bytes).unwrap();
}
