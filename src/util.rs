use std::io;

pub fn single_byte_xor(line: &str) -> io::Result<()> {
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

pub fn single_byte_xor_u8(block: Vec<u8>) -> io::Result<(u8)> {
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

pub fn encrypt_ecb(content: Vec<u8>, keyphrase: Vec<u8>) -> Vec<u8> {
    let mut encoded = Vec::new();

    let key_len = keyphrase.len();
    let mut k = 0;
    for c in content {
        encoded.push(c ^ keyphrase[k]);

        k = k + 1;
        if k == key_len {
            k = 0;
        }
    }

    encoded
}

pub fn decrypt_ecb(cypher: Vec<u8>, keyphrase: Vec<u8>) {
    let mut decoded = Vec::new();

    let key_len = keyphrase.len();
    let mut k = 0;
    for c in cypher {
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

pub fn encrypt_cbc(plain_text: Vec<u8>, key: &[u8; 16], iv: &[u8; 16]) -> Vec<u8> {
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

pub fn decrypt_cbc(cypher: Vec<u8>, key: &[u8; 16], iv: &[u8; 16]) -> Vec<u8> {
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
