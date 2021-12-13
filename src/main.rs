#![feature(int_abs_diff)]
#![feature(slice_as_chunks)]

use std::env;
use std::fs;
use bio::data_structures::suffix_array::{lcp, suffix_array};
use num::integer::gcd;

fn main() {
    let cipher = get_cipher().unwrap();

    let key_len = get_key_len(&cipher);
    let chunks = cipher.chunks_exact(key_len).collect::<Vec<&[u8]>>();
    
    // transpose the original cipher matrix
    let tranposed = (0..key_len)
        .map(|x| chunks.iter().map(|i| i[x]).collect::<Vec<u8>>());

    let freqs_max = tranposed
        .map(|x| get_freq(&x))
        .map(|x| 
            x.into_iter().enumerate().max_by(|x, y| x.1.cmp(&y.1))
        ).collect::<Option<Vec<_>>>().unwrap();
    
    let key = String::from_utf8(freqs_max.iter()
        .map(|&(x, _)| (b'A' + x as u8 + 26 - b'E') % 26 + b'A')
        .collect()).unwrap();
    println!("{:?}", key);
    // println!("Hello, world!");
}

fn get_freq(src: &[u8]) -> [usize; 26] {
    src[..src.len() - 1].iter().fold([0; 26], |mut arr, x| {
        arr[(x - b'A') as usize] += 1;
        arr
    })
}

#[derive(Debug)]
struct LCPCmp {
    pub inner: (usize, isize)
}

impl PartialEq for LCPCmp {
    fn eq(&self, other: &Self) -> bool {
        self.inner.1 == other.inner.1
    }
}

impl PartialOrd for LCPCmp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner.1.partial_cmp(&other.inner.1)
    } 
}

impl Eq for LCPCmp {}

impl Ord for LCPCmp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.1.cmp(&other.inner.1)
    }
}

pub fn top_n<T: Ord>(iter: impl Iterator<Item = T>, n: usize) -> Vec<T> {
    use std::collections::BinaryHeap;
    let mut heap = iter.collect::<BinaryHeap<T>>();
    let mut top_three = Vec::new();
    for _ in 0..n {
       if let Some(v) = heap.pop() {
           top_three.push(v);
       }
    }
    top_three
}

fn gcd_multiple(src: &[usize]) -> usize {
    src.iter().fold(src[0], |cd, &next| {
        gcd(cd, next)
    })
}

fn get_key_len(cipher: &[u8]) -> usize {
    // build a suffix array
    let pos = suffix_array(cipher);
    let lcp = lcp(cipher, &pos);
    
    let top_3 = top_n(lcp.iter().enumerate().map(|inner| LCPCmp { inner }), 3)
        .iter()
        .map(|x| ((pos[x.inner.0], pos[x.inner.0 + 1]), x.inner.1))
        .collect::<Vec<((usize, usize), isize)>>();

    let gaps = top_3
        .iter()
        .map(|(x, _)| x.0.abs_diff(x.1))
        .collect::<Vec<usize>>();
    
    let possible_len = gcd_multiple(&gaps);
    possible_len
}

fn get_cipher() -> Option<Vec<u8>> {
    let cipher_f = env::args().collect::<Vec<String>>();
    let cipher_f = cipher_f.get(1)?;
    let cipher = fs::read_to_string(cipher_f).ok()?;
    let mut res: Vec<u8> = cipher.into();
    res = res.iter().filter(|x| {
        x.is_ascii_alphabetic()
    }).copied().collect::<Vec<u8>>().to_ascii_uppercase();
    res.push(0);
    Some(res)
}
