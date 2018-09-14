use ascii::AsciiStr;
use rand::prelude::*;

use super::util;

fn add_padding(content: String, block_len: usize) -> String {
    let mut out = content.clone();

    let rem = content.len() % block_len;
    if rem != 0 {
        let pad_size = block_len - rem;

        for _ in 0..pad_size {
            // out.push('\x04' as char);
            out.push('-' as char);
        }
    }

    out
}

fn exercise_1() {
    println!("Cryptopals: 2.1");
    println!("Implement PKCS#7 padding");

    let content = String::from("YELLOW SUBMARINE");
    println!("Original content is: {}", content);
    println!("Padded output is: {}", add_padding(content, 20));
}

fn exercise_2() {
    println!("Cryptopals: 2.2");
    println!("Implement CBC mode");

    let vanilla = AsciiStr::from_ascii("Burning 'em, if you ain't quick and nimble I go crazy when I hear a cymbal").unwrap();
    let mut content: Vec<u8> = Vec::new();
    println!("{}", vanilla.len());
    for _ in 0..2 {    // repeat txt twice to prove patterns aren't recognizable
        for vc in 0..vanilla.len() {
            content.push(vanilla[vc] as u8);
        }
    }

    let key: &[u8; 16] = b"YELLOW SUBMARINE";
    let iv: &[u8; 16] = b"INIT_VECTOR_<>_!";

    println!("key length: {:?}", key.len());
    println!("iv length: {:?}", iv.len());
    println!();

    println!("Test plain text in hex");
    for t in &content {
        print!("{:x?} ", *t);
    }

    println!("\n\nEncrypt");
    let cypher = util::encrypt_cbc(content, key, iv);
    for c in &cypher {
        print!("{:x?} ", *c);
    }

    println!("\n\nDecrypt");
    let recovered = util::decrypt_cbc(cypher, key, iv);
    for r in recovered {
        print!("{:x?} ", r);
    }
    println!()
}

fn exercise_3() {
    println!("Cryptopals: 2.3");
    println!("An ECB/CBC detection oracle");

    let orig: &[u8] = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    let iv: &[u8; 16] = b"INIT_VECTOR_<>_!";

    let mut rng = thread_rng();
    let mut key: [u8; 16] = [0; 16];
    for i in 0..key.len() {
        key[i] = rng.gen_range(0,255);
    }

    let mut content: Vec<u8> = Vec::new();
    for _ in 0..rng.gen_range(5,11) {
        content.push(0);
    }
    for o in orig {
        content.push(*o);
    }
    for _ in 0..rng.gen_range(5,11) {
        content.push(0);
    }

    let cypher: Vec<u8>;
    if rng.gen_range(0,2) == 0 {
        cypher = util::encrypt_cbc(content, &key, iv);
        // println!("CBC used");
    } else {
        cypher = util::encrypt_ecb(content, key.to_vec());
        // println!("ECB used");
    }

    let mut check: Vec<u8> = Vec::new();
    for c in 0..4 {
        check.push(cypher[14 + c]);
    }

    let mut repeat = false;
    for c in 18..cypher.len() {
        if check[0] == cypher[c] &&
                check[1] == cypher[c+1] &&
                check[2] == cypher[c+2] &&
                check[3] == cypher[c+3] {
            repeat = true;
        }
    }

    if repeat {
        println!("Most probably ECB");
    } else {
        println!("Most probably CBC");
    }
}

pub fn run(exercise_num: usize) {
    let mut exercises: Vec<&Fn()> = Vec::new();

    exercises.push(&exercise_1);
    exercises.push(&exercise_2);
    exercises.push(&exercise_3);

    if exercise_num > exercises.len() || exercise_num <= 0 {
        println!("Error: exercise number doesn't exist");
        return;
    }

    exercises[exercise_num - 1]();
}
