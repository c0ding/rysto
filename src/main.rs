extern crate hex;
extern crate base64;

fn exercise1_1() {
    println!("Cryptopals: 1.1");
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let dec = hex::decode(hex).unwrap();
    let b64 = base64::encode(&dec);

    println!("{:?}", b64);
}

fn exercise1_2() {
}

fn main() {
    exercise1_1();
}
