use ascii::AsciiStr;

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

fn encrypt_cbc(plain_text: Vec<u8>, key: &[u8; 16], iv: &[u8; 16]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    let mut curr_block = Vec::new();
    let mut prev_block = Vec::new();

    for i in iv {
        prev_block.push(*i);
    }

    let key_len = key.len();
    let mut k: usize = 0;

    for t in plain_text {
        let a: u8 = t ^ prev_block[k];
        let b: u8 = a ^ key[k];
        out.push(b);

        curr_block.push(b);

        // First uses IV
        if k < (key_len - 1) {
            k = k + 1;
        } else {
            k = 0;
            prev_block = curr_block.clone();
            curr_block.clear();
        }
    }

    out
}

fn decrypt_cbc(cypher: Vec<u8>, key: &[u8; 16], iv: &[u8; 16]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    let mut curr_block = Vec::new();
    let mut prev_block = Vec::new();

    for i in iv {
        prev_block.push(*i);
    }

    let key_len = key.len();
    let mut k: usize = 0;

    for c in cypher {
        let a: u8 = c ^ key[k];
        let b: u8 = a ^ prev_block[k];
        out.push(b);
        curr_block.push(c);

        // First uses IV
        if k < (key_len - 1) {
            k = k + 1;
        } else {
            k = 0;
            prev_block = curr_block.clone();
            curr_block.clear();
        }
    }

    out
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
    let cypher = encrypt_cbc(content, key, iv);
    for c in &cypher {
        print!("{:x?} ", *c);
    }

    println!("\n\nDecrypt");
    let recovered = decrypt_cbc(cypher, key, iv);
    for r in recovered {
        print!("{:x?} ", r);
    }
    println!()
}

pub fn run(exercise_num: usize) {
    let mut exercises: Vec<&Fn()> = Vec::new();

    exercises.push(&exercise_1);
    exercises.push(&exercise_2);

    if exercise_num > exercises.len() || exercise_num <= 0 {
        println!("Error: exercise number doesn't exist");
        return;
    }

    exercises[exercise_num - 1]();
}
