mod set1;
mod set2;
mod util;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Error: You need to pass a set and exercise number as arguments");
        println!("Example: rysto 2 1");
        return;
    }

    let mut ex_num: usize = 0;
    match args[2].parse() {
        Ok(num) => {
            ex_num = num;
        },
        Err(_) => {
            println!("Error: second argument needs to be a number");
        }
    }


    match args[1].as_ref() {
        "1" => {
            set1::run(ex_num);
        }
        "2" => {
            set2::run(ex_num);
        }
        _ => {
            println!("Error: first argument isn't a valid set");
        }
    }
}
