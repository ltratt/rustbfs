use std::{env, error, fs, io::{self, Read, Write}};
fn main() -> Result<(), Box<dyn error::Error>> {
  let prog = fs::read(env::args().nth(1).unwrap())?;
  let mut pc = 0;
  let mut bmap = vec![];
  let mut bstack = vec![];
  for (i, b) in prog.iter().enumerate() {
    bmap.push(usize::max_value());
    if *b as char == '[' { bstack.push(i); }
    else if *b as char == ']' {
      let s = bstack.pop().unwrap();
      bmap[s] = i;
      bmap[i] = s;
    }
  }
  let mut cells = vec![0u8; 10000];
  let mut cc = 0;
  while pc < prog.len() {
    match prog[pc] as char {
      '<' => cc -= 1,
      '>' => cc += 1,
      '+' => cells[cc] = cells[cc].wrapping_add(1),
      '-' => cells[cc] = cells[cc].wrapping_sub(1),
      '[' if cells[cc] == 0 => pc = bmap[pc],
      ']' if cells[cc] != 0 => pc = bmap[pc],
      '.' => io::stdout().write_all(&cells[cc..cc+1])?,
      ',' => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
      _ => ()
    }
    pc += 1;
  }
  Ok(())
}
