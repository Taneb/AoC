extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;
use std::str;
use std::char;

fn extract(input: &[u8]) -> Option<(u8,u8)> {
    assert!(input.len() >= 3);
    if input[0] != 0 {return None};
    if input[1] != 0 {return None};
    if input[2] >= 16 {return None};
    let a = input[2];
    let b = (input[3] & 0xF0) >> 4;
    return Some((a,b));
}

fn extend(base: &Vec<u8>, index: usize) -> Vec<u8> {
    let mut rep = Vec::new();
    if index == 0 {
        rep.push(48);
    } else {
        let mut ix = index;
        while ix > 0 {
            rep.push(48 + (ix % 10) as u8);
            ix /= 10;
        }
        rep.reverse();
    }
    let mut answer = base.clone();
    answer.extend_from_slice(rep.as_slice());
    return answer;
}

fn main() {
    let mut md5 = Md5::new();
    let mut base = Vec::new();
    base.extend_from_slice(b"uqwqemis");
    let mut index = 0;
    let mut count = 0;
    let mut out1 = String::from("________");
    let mut out2 = String::from("________");
    loop {
        let mut out = vec![0;16];
        let input = extend(&base, index);
        md5.input(input.as_slice());
        md5.result(&mut out.as_mut_slice());
        md5.reset();
        index += 1;
        match extract(out.as_slice()) {
            Some(n) => {
                // boring part
                if count < 8 {
                    out1.remove(count);
                    out1.insert(
                        count,
                        char::from_digit(n.0 as u32, 16).unwrap()
                    );
                    count += 1;
                }

                // cool part
                if n.0 > 7 {
                    continue;
                }

                match out2.remove(n.0 as usize) {
                    '_' =>
                        out2.insert(
                            n.0 as usize,
                            char::from_digit(n.1 as u32, 16).unwrap()
                        ),
                    d => out2.insert(n.0 as usize, d)
                };
            }
            None => {}
        };
        if !out2.chars().any(|c| c == '_') {
            break;
        }
    }
    println!("Yawn: {}", out1);
    println!("Cool: {}", out2);
}
