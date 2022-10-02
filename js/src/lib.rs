use std::cmp;
use std::ops::BitXorAssign;
use std::fmt::Write;
use wasm_bindgen::prelude::*;

pub struct RollingHash {
    x: u32,
    y: u32,
    z: u32,
    c: u32,
    window: [u32; 8],
    size: u32,
    hash: u32
}
impl RollingHash {

    fn init() -> RollingHash {
        RollingHash {
            x: 0,
            y: 0,
            z: 0,
            c: 0,
            window: [0; 8],
            size: 8,
            hash: 0
        }
    }

    fn update(&mut self, d: u8) {
        self.y = self.y.wrapping_sub(self.x); // y = y - x
        self.y = self.y.wrapping_add(self.size.wrapping_mul(d as u32)); // y = y + size * d
        self.x = self.x.wrapping_add(d as u32); // x = x + d
        self.x = self.x.wrapping_sub(self.window[(self.c % self.size) as usize]); // x = x - window[c % size]
        self.window[(self.c % self.size) as usize] = d as u32; // window[c % size] = d
        self.c = self.c.wrapping_add(1); // c = c + 1
        self.z = self.z << 5; // z = z << 5
        self.z ^= d as u32; // z = z ^ d
        self.hash = self.x.wrapping_add(self.y).wrapping_add(self.z) // return (x + y + z)
    }
}

pub struct FNVHash {
    hash: u64,
}

impl FNVHash {
    fn init() -> FNVHash {
        FNVHash {
            hash: 14695981039346656037
        }
    }

    fn update(&mut self, b: u8) {
        self.hash = self.hash.wrapping_mul(1099511628211);
        self.hash.bitxor_assign(b as u64);
    }
}

fn to_base64(idx: &u8) -> char {
    return ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'][*idx as usize];
}
fn wagner_fischer(a: &str, b: &str) -> u32 {
    let mut matrix = vec![vec![0; b.len()]; a.len()];
    let mut a_chars = a.chars();
    let mut b_chars = b.chars();
    for i in 1..=a.len() {
        matrix[i][0] = i as u32;
    }
    for i in 1..=b.len() {
        matrix[0][i] = i as u32;
    }
    for j in 1..b.len() {
        for i in 1..a.len() {
            let mut substitution_cost = 0;
            if a_chars.nth(i) == b_chars.nth(j) {
                substitution_cost = 1;
            }
            matrix[i][j] = cmp::min(
                matrix[i - 1][j] + 1,
                cmp::min (
                    matrix[i][j - 1] + 1,
                    matrix[i - 1][j - 1] + substitution_cost
                )
            )
        }
    }
    return matrix[a.len() - 1][b.len() - 1];
}
#[wasm_bindgen]
pub struct MethodSum {
    block_size: u32,
    result: Option<String>,
    method_insn: Vec<u8>
}
#[wasm_bindgen]
impl MethodSum {
    #[wasm_bindgen(constructor)]
    pub fn init(insns: Vec<u8>) -> MethodSum {
        MethodSum {
            block_size: 32 * 2_u32.pow((insns.len() as f64 / (32.0 * 16.0)).log2().floor() as u32),
            result: None,
            method_insn: insns
        }
    }

    pub fn compare(a: &str, b: &str) -> u32 {
        let a_parts = a.split(":").collect::<Vec<_>>();
        let b_parts = b.split(":").collect::<Vec<_>>();
        let first_part_blocks: u8 = a_parts[0].parse().unwrap();
        let second_part_blocks: u8 = b_parts[0].parse().unwrap();
        if first_part_blocks == second_part_blocks {
            let close= cmp::min(wagner_fischer(a_parts[1], b_parts[1]), wagner_fischer(a_parts[2], b_parts[2]));
            let result: f64 = 100.0 - ((100.0 * 32.0 * close as f64)/(64.0 * (a.len() + b.len()) as f64));
            return result.clamp(0.0, 100.0).floor() as u32;
        } else if first_part_blocks == 2 * second_part_blocks {
            let close = wagner_fischer(a_parts[1], b_parts[2]);
            let result: f64 = 100.0 - ((100.0 * 32.0 * close as f64)/(64.0 * (a.len() + b.len()) as f64));
            return result.clamp(0.0, 100.0).floor() as u32;
        } else if first_part_blocks * 2 == second_part_blocks {
            let close = wagner_fischer(a_parts[2], b_parts[1]);
            let result: f64 = 100.0 - ((100.0 * 32.0 * close as f64)/(64.0 * (a.len() + b.len()) as f64));
            return result.clamp(0.0, 100.0).floor() as u32;
        }
        return 0;
    }

    pub fn get_hash(&mut self) -> String {
        match self.result {
            Some(..) => (),
            None => {
                let mut done = false;
                while !done {
                    let mut hash = RollingHash::init();
                    let mut h1 = FNVHash::init();
                    let mut h2 = FNVHash::init();
                    let mut sig1 = String::from("");
                    let mut sig2 = String::from("");
                    for b in &self.method_insn {
                        hash.update(*b);
                        h1.update(*b);
                        h2.update(*b);
                        if hash.hash % (self.block_size) == self.block_size.wrapping_sub(1) as u32 {
                            write!(sig1, "{}", to_base64(&((h1.hash % 64 ) as u8))).unwrap();
                            h1 = FNVHash::init();
                        }
                        if hash.hash % (2 * self.block_size) == self.block_size.wrapping_mul(2).wrapping_sub(1) as u32 {
                            write!(sig2, "{}", to_base64(&((h2.hash % 64 ) as u8))).unwrap();
                            h2 = FNVHash::init();
                        }
                    }

                    if sig1.len() < 16 { // 32 (S)/ 2
                        if self.block_size == 1 {
                            panic!("Can't halve the block size any more");
                        }
                        self.block_size = self.block_size / 2;

                    } else {
                        done = true;
                        let result = format!("{}:{}:{}", self.block_size, sig1, sig2);
                        self.result = Option::from(result);
                    }
                }
            }
        }
        return self.result.as_ref().unwrap().to_string();
    }
}