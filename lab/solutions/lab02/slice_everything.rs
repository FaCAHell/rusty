fn main() {
    let my_bites: Vec<u8> = vec![
        0x01, 0x02, 0x03, 0x04, 
        0x05, 0x06, 0x07, 0x08, 
        0x09, 0x0A, 0x0B, 0x0C, 
        0x0D, 0x0E, 0x0F, 0x10,
        0x11,
    ];

    let my_slices = slicer(&my_bites[..], 4);
    println!("{:#02X?}", my_slices);
}

fn slicer(bites: &[u8], size: usize) -> Vec<&[u8]> {
  
    let mut slices: Vec<&[u8]> = Vec::new();
    let mut start = 0;
    while start < bites.len() {
        let end = (start + size).min(bites.len());
        slices.push(&bites[start..end]);
        start += size;
    }

    slices
}
