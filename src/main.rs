extern crate hex;
extern crate base64;

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

fn exercise1_3() {
    println!("Cryptopals: 1.3");
    println!("Single-byte XOR cipher");

    let hex_str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let dec = hex::decode(hex_str).unwrap();

    for x in 65..122{
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
            let mut char_vec = Vec::new();
            for c in chars {
                char_vec.push(c as char);
            }
            let s: String = char_vec.into_iter().collect();

            println!("{:?}", s);
        }
    }
}

fn main() {
    exercise1_3();
}
