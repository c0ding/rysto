mod set1;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: You need to pass an exercise number as the first argument");
        return;
    }

    match args[1].as_ref() {
        "1" => {
            set1::exercise_1();
        }
        "2" => {
            set1::exercise_2();
        }
        "3" => {
            set1::exercise_3();
        }
        "4" => {
            set1::exercise_4();
        }
        "5" => {
            set1::exercise_5();
        }
        "6" => {
            set1::exercise_6();
        }
        "7" => {
            set1::exercise_7();
        }
        "8" => {
            set1::exercise_8();
        }
        _ => {
            println!("Error: Invalid exercise number");
        }
    }
}
