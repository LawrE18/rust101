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
/*    let mut s: String = "".to_string();
    let mut p: String = "\\\n,.\t0!".to_string();
    for c in v {
        let flag = if c != 10 { true } else { false };
        if flag & c.is_ascii_alphabetic() {
            print!("[{}]", c as char);
            //s.push(c as char);
        } else if p.find(c as char) != None {
            print!("[{:02X?}]", c as char);
            //s.push_str("\\n");
        } else {
            print!(".")
            //s.push('{');
            //s.push_str(c.to_string().as_str());
            //s.push('}');
        }
    }
    println!("{}", s);*/
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
}