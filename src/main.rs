extern crate hex;
extern crate base64;
extern crate ascii;
extern crate crypto;

use std::env;

use std::io;
use std::io::BufReader;
use std::fs::File;
// use std::io::Read;
use std::io::BufRead;

use ascii::AsciiStr;

use crypto::aes;
use crypto::buffer::{ WriteBuffer, BufferResult, ReadBuffer };

fn exercise1_1() {
    println!("Cryptopals: 1.1");
    println!("Convert hex to base64");

    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let dec = hex::decode(hex).unwrap();
    // println!("{:?}", dec);
    let b64 = base64::encode(&dec);

    println!("{:?}", b64);
}

fn exercise1_2() {
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

fn single_byte_xor(line: &str) -> io::Result<()> {
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

fn single_byte_xor_u8(block: Vec<u8>) -> io::Result<(u8)> {
    let mut got_ret = false;
    let mut ret: u8 = 0;
    let mut max_seen_letters = 0;

    for x in 30..160{
        let mut found = true;
        let mut chars = Vec::new();

        for a in block.iter() {
            let tmp_dec = *a ^ x;
            if tmp_dec < 10 || tmp_dec > 126 || (tmp_dec > 14 && tmp_dec < 32) {  // printable ascii characters
                found = false;
                // println!("Got a wrong thing: {:?}", tmp_dec);
                break;
            }

            chars.push(tmp_dec);
        }

        if found {
            got_ret = true;

            let mut char_vec = Vec::new();
            let mut seen_letters = 0;
            for c in chars {
                if (c > 97 && c < 122) || c == 32 || c == 46 || c == 44  {
                     seen_letters = seen_letters + 1;
                }

                char_vec.push(c as char);
            }

            // let s: String = char_vec.into_iter().collect();

            if seen_letters > max_seen_letters {
                ret = x;
                max_seen_letters = seen_letters;
            }
        }
    }

    if got_ret {
        return Ok(ret);
    } else {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "No valid strings"));
    }
}

fn exercise1_3() {
    println!("Cryptopals: 1.3");
    println!("Single-byte XOR cipher");

    let hex_str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let _ = single_byte_xor(hex_str);
}

fn exercise1_4() {
    println!("Cryptopals: 1.4");
    println!("Detect single-character XOR");

    let f = File::open("4.txt").unwrap();
    let file = BufReader::new(&f);
    for (num, line) in file.lines().enumerate() {
        let l = line.unwrap();
        match single_byte_xor(&l) {
            Ok(_) => {
                println!("<- {:?}", num);
            }
            Err(_) => ()
        }
    }
}

fn exercise1_5() {
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

fn _hamming_distance(first: &str, second: &str) -> u8 {
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

fn hamming_distance_vec(first: &Vec<&u8>, second: &Vec<&u8>) -> u64 {
    let mut count: u64 = 0;

    // let mut len = first.len();
    // if len > second.len() {
    //     len = second.len();
    // }

    // TODO: Use len instead of 300
    for c in 0..300 {
        // println!("here {:?} {:?} {:?}", c, first.len(), second.len());
        count = count + differing_bits(*first[c], *second[c]) as u64;
    }
    // println!("{:?} ", count);

    count
}

fn lowest_key_size(content: &Vec<u8>) -> u8 {
    let mut count: u64;
    let mut first: bool;
    let mut first_vec = Vec::new();
    let mut second_vec = Vec::new();
    let mut lowest_ks: u8 = 0;
    let mut lowest_dist_seen = 1000;

    for ks in 2..50 {
        // Read blocks based on keysize into vector A and B then calculate
        // hamming distance
        first_vec.clear();
        second_vec.clear();

        count = 0;
        first = true;
        for i in content {
            // println!("{:?}", byte.unwrap());

            if first {
                first_vec.push(i);
            } else {
                second_vec.push(i);
            }

            count = count + 1;
            if count == ks {
                if !first {
                    // println!("s {:?}", count);
                    first = true;
                } else {
                    // println!("f {:?}", count);
                    first = false;
                }
                count = 0;
            }
        }

        let dist = hamming_distance_vec(&first_vec, &second_vec);
        // let dist = 0;
        if lowest_dist_seen > dist {
            // println!("{:?} {:?}", ks, dist);
            lowest_dist_seen = dist;
            lowest_ks = ks as u8;
        }
    }

    // let dist = hamming_distance(l.slice());
    // println!("{:?} {:?}", ks, dist);

    lowest_ks
}

fn decrypt(content: Vec<u8>, keyphrase: Vec<u8>) {
    let mut decoded = Vec::new();

    let key_len = keyphrase.len();
    let mut k = 0;
    for c in content {
        // print!("{:?}", (c ^ keyphrase[k]) as char);
        decoded.push((c ^ keyphrase[k]) as char);

        k = k + 1;
        if k == key_len {
            k = 0;
        }
    }

    let d: String = decoded.into_iter().collect();
    println!("Decoded content:");
    println!("{}", d);
}

fn exercise1_6() {
    println!("Cryptopals: 1.6");
    println!("Implement repeating-key XOR");
    let f = File::open("6.txt").unwrap();
    let file = BufReader::new(&f);
    let mut content = Vec::new();
    for line in file.lines() {
        let hex = base64::decode(&line.unwrap()).unwrap();
        for i in hex {
            content.push(i);
        }
    }

    let ks = lowest_key_size(&content);
    println!("key size: {:?}", ks);

    let mut keyphrase = Vec::new();
    let mut keyphrase_u8 = Vec::new();

    for i in 0..ks {
        let mut count = 0;
        let mut block = Vec::new();
        for b in 0..(content.len() - ks as usize) {
            if count == 0 {
                block.push(content[b + i as usize]);
            }

            count = count + 1;
            if count == 29 {
                count = 0;
            }
        }
        match single_byte_xor_u8(block) {
            Ok(k) => {
                // println!("{:?} -> {:?}", i, k);
                keyphrase.push(k as char);
                keyphrase_u8.push(k);
            },
            Err(_) => {}
        }
    }

    let k: String = keyphrase.into_iter().collect();
    println!("Keyphrase: {:?}", k);

    decrypt(content, keyphrase_u8);
}

fn exercise1_7() {
    println!("Cryptopals: 1.7");
    println!("AES in ECB mode");

    let f = File::open("7.txt").unwrap();
    let file = BufReader::new(&f);
    let mut content = Vec::new();
    for line in file.lines() {
        let hex = base64::decode(&line.unwrap()).unwrap();
        for i in hex {
            content.push(i);
        }
    }
    println!("Content has size: {}", content.len());

    let key: &[u8] = b"YELLOW SUBMARINE";

    let mut decryptor = aes::ecb_decryptor(aes::KeySize::KeySize128,
                                           key,
                                           crypto::blockmodes::PkcsPadding);

    let mut final_result = Vec::new();
    let mut buffer = [0; 2880];
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(&content);
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

    let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
    match result {
        BufferResult::BufferUnderflow => {
            for c in write_buffer.take_read_buffer().take_remaining() {
                final_result.push(*c as char);
            }
        },
        BufferResult::BufferOverflow => {
            println!("Buffer not big enough");
        }
    }

    let fr: String = final_result.into_iter().collect();
    println!("{}", fr);
}

fn exercise1_8() {
    println!("Cryptopals: 1.8");
    println!("Detect AES in ECB mode");

    let f = File::open("8.txt").unwrap();
    let file = BufReader::new(&f);
    let mut content = Vec::new();
    let mut repeat: u32;
    let mut max_seen_repeat = 0;
    for (lnum, line) in file.lines().enumerate() {
        repeat = 0;
        let hex = base64::decode(&line.unwrap()).unwrap();
        for i in hex {
            content.push(i);
        }

        let mut block = Vec::new();
        for b in (0..(content.len() - 16)).step_by(16) {
            for i in 0..16 {
                block.push(content[b + i]);
            }
            // println!("{:?}", block);

            for c in ((b+16)..(content.len() - 16)).step_by(16) {
                for tmp in 0..17 {
                    if tmp == 16 {
                        // println!("{:x?}", block);
                        // println!("{:?}", base64::encode(&block));
                        repeat = repeat + 1;
                        break;
                    }

                    if block[tmp] != content[c + tmp] {
                        break;
                    }
                }
            }

            block.clear();
        }

        if repeat > max_seen_repeat {
            max_seen_repeat = repeat;
            println!("ECB line num: {}", lnum);
        }

        content.clear();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: You need to pass an exercise number as the first argument");
        return;
    }

    match args[1].as_ref() {
        "1" => {
            exercise1_1();
        }
        "2" => {
            exercise1_2();
        }
        "3" => {
            exercise1_3();
        }
        "4" => {
            exercise1_4();
        }
        "5" => {
            exercise1_5();
        }
        "6" => {
            exercise1_6();
        }
        "7" => {
            exercise1_7();
        }
        "8" => {
            exercise1_8();
        }
        _ => {
            println!("Error: Invalid exercise number");
        }
    }
}
