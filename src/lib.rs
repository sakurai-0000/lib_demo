mod generator;
pub fn print_random_number(){
  let n = generator::gen_run();
  print!("Random u8: {}", n);
}