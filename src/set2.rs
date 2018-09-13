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

pub fn run(exercise_num: usize) {
    let mut exercises: Vec<&Fn()> = Vec::new();

    exercises.push(&exercise_1);

    if exercise_num > exercises.len() || exercise_num <= 0 {
        println!("Error: exercise number doesn't exist");
        return;
    }

    exercises[exercise_num - 1]();
}
