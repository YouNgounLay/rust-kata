fn main() {
    let three = 0b11;
    let thirty = 0o36; // 0o prefix indicates octal numerals.
    let three_hundred = 0x12c;
    
    println!("Base 10: {} {} {}", three, thirty, three_hundred);
    println!("Base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("Base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("Base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}
