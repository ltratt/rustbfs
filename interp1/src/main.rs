use std::{env, error, fs, io::{self, Read, Write}};
fn main() -> Result<(), Box<dyn error::Error>> {
  let prog = fs::read(env::args().nth(1).unwrap())?;
  let mut pc = 0;
  let mut cells = vec![0u8; 10000];
  let mut cc = 0;
  while pc < prog.len() {
    match prog[pc] as char {
      '<' => cc -= 1,
      '>' => cc += 1,
      '+' => cells[cc] = cells[cc].wrapping_add(1),
      '-' => cells[cc] = cells[cc].wrapping_sub(1),
      '[' if cells[cc] == 0 => {
        let mut count = 1;
        while count > 0 {
          pc += 1;
          if prog[pc] as char == ']' { count -= 1 }
          else if prog[pc] as char == '[' { count += 1 }
        }
      }
      ']' if cells[cc] != 0 => {
        let mut count = 1;
        while count > 0 {
          pc -= 1;
          if prog[pc] as char == '[' { count -= 1 }
          else if prog[pc] as char == ']' { count += 1 }
        }
      }
      '.' => io::stdout().write_all(&cells[cc..cc+1])?,
      ',' => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
      _ => ()
    }
    pc += 1;
  }
  Ok(())
}
