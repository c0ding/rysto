fn exercise_1() {
    println!("Cryptopals: 2.1");
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
