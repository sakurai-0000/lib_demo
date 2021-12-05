use rand::Rug;
pub fn gen_run() -> u8 {
 let mut rng = rang::thread_rng();
 let n: u8 = rng.gen();
 n
}