// Premitive Data Types in Rust
// int, float, char, bool

fn main() {
    // Integer Types
    // Rust has signed (+ and -) and unsigned Integer (only +) types of different sizes.
    // i8, i16, i32, i64, i128, isize (signed)
    // u8, u16, u32, u64, u128, usize (unsigned)
    let x: i32 = -42;
    let y: u64 = 100;

    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    // Maximum and Minimum Values for i32
    println!("Maximum i32: {}", i32::MAX);
    println!("Minimum i32: {}", i32::MIN);

    // Maxumum and Minimum Values for u64
    println!("Maximum u64: {}", u64::MAX);
    println!("Minimum u64: {}", u64::MIN);


    // ===================================

    // Floating Point Types
    // Rust has two floating point types: f32 and f64.
    let a: f32 = 3.14;
    let b: f64 = 2.718281828459045;
    println!("Floating Point f32: {}", a);
    println!("Floating Point f64: {}", b);

    // ===================================

    // Boolean values
    let is_snowing: bool = true;

    println!("Is it snowing? {}", is_snowing);

    // ===================================

    // Character Type
    // Rust's char type represents a single Unicode scalar value.
    let letter: char = 'R';
    let emoji: char = '😊';
    println!("Character: {}", letter);
    println!("Emoji: {}", emoji);
    
    // Unicode scalar values are 4 bytes in size, allowing for a wide range of characters
    // including letters, numbers, symbols, and emojis.
    // Example of a Unicode character
    let unicode_char: char = '𐍈'; // Gothic letter hwair
    println!("Unicode Character: {}", unicode_char);
}