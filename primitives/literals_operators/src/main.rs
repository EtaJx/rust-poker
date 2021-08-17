fn main() {
  // Interger addition
  println!("1 + 2 = {}", 1u32 + 2);

  // Interger subtraction
  println!("1 - 2 = {}", 1i32 - 2);

  // Short-circuiting boolean logic
  println!("ture AND false is {}", true && false);
  println!("ture OR false is {}", true || false);
  println!("NOT ture is {}", !true);

  // Bitwise operations
  println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
  println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
  println!("001 XOR 0101 is {:04b}", 0b0011u32 ^ 0b01010);
  println!("1 << 5 is {}", 1u32 << 5);
  println!("0x80 >> 2 is 0x{:x}", 0x8032 >> 2);

  // Use underscores to improve readability!
  println!("one milloin is written as {}", 1_000_000u32);
}
