use std::borrow::Borrow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::io::Write; // handle files
use std::str;
use std::io;
use std::iter::zip;
use std::ops::Deref;
use hex::ToHex;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn strxor(l1: &str, l2: &str) -> Vec<u8> {
    let v: Vec<u8>;
    let mut a = hex::decode(l1.clone()).unwrap();
    let mut b = hex::decode(l2.clone()).unwrap();
    if a.len() > b.len() {
        /*for (x, y) in zip(&a[..b.len()], b.clone()) {
            print!("{}", (x ^ y).to_ascii_uppercase())
        }
        println!();*/
        zip(&a[..b.len()], b.clone()).into_iter().map(|(x, y)| x ^ y).collect::<Vec<_>>()
    }
    else {
        /*for (x, y) in zip(a.clone(), &b[..a.len()]) {
            print!("{}", (x ^ y) as char)
        }
        println!();*/
        zip(a.clone(), &b[..a.len()]).into_iter().map(|(x, y)| x ^ y).collect::<Vec<_>>()
    }
}

fn print(v: Vec<u8>) {
    /*let mut ans: String = "".to_string();
    for c in v {
        print!("{:?}", c);
    }*/
    print!("{}", hex::encode(v));
}

fn variants(v: Vec<Vec<u8>>) {
    let mut h: HashMap<usize, Vec<char>> = HashMap::new();
    for (i, c) in v.iter().enumerate() {
        for (j, d) in c.iter().enumerate() {
            if d.is_ascii_alphabetic() {
                match h.entry(j) {
                    Entry::Vacant(e) => { e.insert(vec![*d as char]); },
                    Entry::Occupied(mut e) => { e.get_mut().push(*d as char); },
                };
            } else {
                continue;
            }
        }
    }
    for i in 0..h.len() {
        match h.get(&i) {
            Some(v) => {
                let mut vh: HashMap<char, i32> = HashMap::new();
                for val in v.clone() {
                    match vh.entry(val) {
                        Entry::Vacant(mut e) => { e.insert(1); },
                        Entry::Occupied(mut e) => { let count = e.get_mut(); *count += 1; },
                    }
                }
                if vh.len() >= 4 {
                    print!("_");
                } else {
                    let mut max_c: char = '\0';
                    for (key, val) in &vh {
                        if val >= vh.get(key).unwrap() {
                            max_c = *key;
                        } else {
                            continue;
                        }
                    }
                    print!("{}", max_c);
                }

            },
            None => {}
        };
    }
    
}

pub fn main() {
    let content = fs::read_to_string("in.txt").expect("Something went wrong");
    let mut ans: String = "".to_string();
    for (i, a) in content.lines().enumerate() {
        println!("{} ", i);
        let mut v: Vec<Vec<u8>> = vec![];
        for (j, b) in content.lines().enumerate() {
            if i != j {
                v.push(strxor(&a, &b));
            } else {
                continue;
            }
        }
        variants(v);
        println!();
    }
    let file = File::open(Path::new("in.txt")).unwrap();
    let reader = BufReader::new(&file);
    let m: String = "THE CIPHERTEXT PRODUCED BY A WEAK ENCRYPTION ALGORITHM LOOKS AS GOOD AS CIPHERTEXT ".to_string().encode_hex();
    let vl: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let key = hex::encode(strxor(&m, &vl[2]));
    print(strxor(&key, &vl[10]));
}