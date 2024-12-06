fn main() {
    // Scalar Types

    // Integer Types
    // signed: i8, i16, i32, i64, i128, isize
    // -(2^(n-1)) to (2^(n-1) - 1)
    //
    // unsigned: u8, u16, u32, u64, u128, usize
    // 0 to 2^n - 1
    //
    //  isize and usize depend on the kind of computer your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.
    let x: u8 = 255;
    println!("La valeur de x est: {x}.")
}
