#![allow(warnings)]
mod e1;
mod e2;
fn main() {

    let data = b"123456789";
    for i in data {
        println!("{i}")
    }
    assert_eq!(calculate_crc(data), 0xCBF43926);
    // Test empty input
    //assert_eq!(calculate_crc(&[]), 0x00000000);

    // Test single byte
    //assert_eq!(calculate_crc(&[0x61]), 0xEDB88320);  // CRC-32 of 'a'
}


const fn make_crc_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut i = 0;
    while i < 256 {
        let mut c = i as u32;
        let mut j = 0;
        while j < 8 {
            if c & 1 != 0 {
                c = 0xEDB88320 ^ (c >> 1);
            } else {
                c >>= 1;
            }
            j += 1;
        }
        table[i] = c;
        i += 1;
    }
    table
}

static CRC_TABLE: [u32; 256] = make_crc_table();

fn calculate_crc(data: &[u8]) -> u32 {
    let mut crc = 0xFFFFFFFFu32;
    for &byte in data {
        let index = ((crc ^ u32::from(byte)) & 0xFF) as usize;
        crc = CRC_TABLE[index] ^ (crc >> 8);
    }
    !crc
}
