use std::{env, error, fs, io::{self, Read, Write}};
enum Ops {
  Left(usize), Right(usize), Add(u8), Sub(u8), LBrack(usize), RBrack(usize), Zero, Output, Input
}
fn main() -> Result<(), Box<dyn error::Error>> {
  let prog = compile()?;
  evaluate(prog)?;
  Ok(())
}
// Compiler
fn optimise(prog: &mut Vec<Ops>) {
  let mut i = 0;
  while i < prog.len() {
    if i + 2 < prog.len() && matches!(prog[i..i+3], [Ops::LBrack(_), Ops::Sub(1), Ops::RBrack(_)]) {
      prog.splice(i..i+3, [Ops::Zero]);
      i += 3;
    } else { i += 1; }
  }
}
fn compile() -> Result<Vec<Ops>, Box<dyn error::Error>> {
  let mut prog = vec![];
  let bytes = fs::read(env::args().nth(1).unwrap())?;
  let mut i = 0;
  while i < bytes.len() {
    match bytes[i] as char {
      '<' => {
        let j = bytes[i..].iter()
          .take_while(|b| **b as char == '<').count();
        prog.push(Ops::Left(j));
        i += j - 1;
      }
      '>' => {
        let j = bytes[i..].iter()
          .take_while(|b| **b as char == '>').count();
        prog.push(Ops::Right(j));
        i += j - 1;
      }
      '+' => {
        let j = bytes[i..].iter()
          .take_while(|b| **b as char == '+').count();
        prog.push(Ops::Add(u8::try_from(j).unwrap()));
        i += j - 1;
      }
      '-' => {
        let j = bytes[i..].iter()
          .take_while(|b| **b as char == '-').count();
        prog.push(Ops::Sub(u8::try_from(j).unwrap()));
        i += j - 1;
      }
      '[' => prog.push(Ops::LBrack(usize::max_value())),
      ']' => prog.push(Ops::RBrack(usize::max_value())),
      '.' => prog.push(Ops::Output),
      ',' => prog.push(Ops::Input),
      _ => (),
    }
    i += 1;
  }
  optimise(&mut prog);
  let mut bstack = vec![];
  let mut i = 0;
  while i < prog.len() {
    match prog[i] {
      Ops::LBrack(_) => bstack.push(i),
      Ops::RBrack(_) => {
        let s = bstack.pop().unwrap();
        prog[s] = Ops::LBrack(i);
        prog[i] = Ops::RBrack(s);
      }
      _ => ()
    }
    i += 1;
  }
  Ok(prog)
}
// Evaluator / "Interpreter"
fn evaluate(prog: Vec<Ops>) -> Result<(), Box<dyn error::Error>> {
  let mut cells = vec![0u8; 10000];
  let mut cc = 0usize;
  let mut pc = 0;
  while pc < prog.len() {
    match prog[pc] {
      Ops::Left(i) => cc -= i,
      Ops::Right(i) => cc += i,
      Ops::Add(i) => cells[cc] = cells[cc].wrapping_add(i),
      Ops::Sub(i) => cells[cc] = cells[cc].wrapping_sub(i),
      Ops::LBrack(i) if cells[cc] == 0 => pc = i,
      Ops::RBrack(i) if cells[cc] != 0 => pc = i,
      Ops::Zero => cells[cc] = 0,
      Ops::Output => io::stdout().write_all(&cells[cc..cc+1])?,
      Ops::Input => io::stdin().read_exact(&mut cells[cc..cc + 1])?,
      _ => ()
    }
    pc += 1;
  }
  Ok(())
}
