// ILLEGAL
fn main() {
    let _r1 = 3u8..12i8;
    let _r2: std::ops::Range<u32> = -3..12;
    let _r3: std::ops::Range<i32> = 3i16..12;
}
