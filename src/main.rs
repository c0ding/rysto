extern crate hex;
extern crate base64;
extern crate ascii;

use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

use ascii::AsciiStr;

fn _exercise1_1() {
    println!("Cryptopals: 1.1");
    println!("Convert hex to base64");

    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let dec = hex::decode(hex).unwrap();
    // println!("{:?}", dec);
    let b64 = base64::encode(&dec);

    println!("{:?}", b64);
}

fn _exercise1_2() {
    println!("Cryptopals: 1.2");
    println!("Fixed XOR");

    let str_a = "1c0111001f010100061a024b53535009181c";
    let str_b = "686974207468652062756c6c277320657965";

    let hex_a = hex::decode(str_a).unwrap();
    let hex_b = hex::decode(str_b).unwrap();

    let mut c = 0;
    for a in hex_a.iter() {
        let tmp_dec = a ^ hex_b[c];
        print!("{:x?}", tmp_dec);
        c = c + 1;
    }
    println!();
}

fn _single_byte_xor(line: &str) -> io::Result<()> {
    let mut ret = false;
    let dec = hex::decode(line).unwrap();

    for x in 32..126{
        let mut found = true;
        let mut chars = Vec::new();

        for a in dec.iter() {
            let tmp_dec = a ^ x;
            if tmp_dec < 32 || tmp_dec > 126 {  // printable ascii characters
                found = false;
                break;
            }

            chars.push(tmp_dec);
        }

        if found {
            ret = true;

            let mut char_vec = Vec::new();
            for c in chars {
                // println!("{:?}", c as u8);
                char_vec.push(c as char);
            }

            let s: String = char_vec.into_iter().collect();

            println!("{:?}", s);
        }
    }

    if ret {
        return Ok(());
    } else {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "No valid strings"));
    }
}

fn _exercise1_3() {
    println!("Cryptopals: 1.3");
    println!("Single-byte XOR cipher");

    let hex_str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let _ = _single_byte_xor(hex_str);
}

fn _exercise1_4() {
    println!("Cryptopals: 1.4");
    println!("Detect single-character XOR");

    let f = File::open("4.txt").unwrap();
    let file = BufReader::new(&f);
    for (num, line) in file.lines().enumerate() {
        let l = line.unwrap();
        match _single_byte_xor(&l) {
            Ok(_) => {
                println!("<- {:?}", num);
            }
            Err(_) => ()
        }
    }
}

fn _exercise1_5() {
    println!("Cryptopals: 1.5");
    println!("Implement repeating-key XOR");

    let vanilla = AsciiStr::from_ascii("Burning 'em, if you ain't quick and nimble I go crazy when I hear a cymbal").unwrap();
    let mut ice = Vec::new();
    ice.push(73);   // I
    ice.push(67);   // C
    ice.push(69);   // E

    let mut ic = 0;
    let van_len = vanilla.len();
    for vc in 0..van_len {
        print!("{:x?}", (vanilla[vc] as u16) ^ ice[ic]);
        ic = ic + 1;
        if ic == 3 {
            ic = 0;
        }
    }
    println!();
}

fn differing_bits(first: u8, second: u8) -> u8 {
    let mut count = 0;
    for b in 0..8 {
        // println!("{:?}", 1 << b);
        let f = first & (1 << b);
        let s = second & (1 << b);
        if f != s {
            count = count + 1;
        }
    }
    // println!("{:?}", count);

    count
}

fn hamming_distance(first: &str, second: &str) -> u8 {
    let first_asc = AsciiStr::from_ascii(first).unwrap();
    let second_asc = AsciiStr::from_ascii(second).unwrap();

    // println!("first len: {:?}", first_asc.len());
    // println!("first len: {:?}", second_asc.len());

    let mut count = 0;
    for c in 0..first_asc.len() {
        count = count + differing_bits(first_asc[c] as u8, second_asc[c] as u8);
    }
    // println!("{:?} ", count);

    count
}

fn exercise1_6() {
    let dist = hamming_distance("this is a test", "wokka wokka!!!");
    println!("{:?}", dist);
}

fn main() {
    exercise1_6();
}
