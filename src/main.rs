use std::env;

fn main() {

if env::args().len() <= 2 {
    println!("We need at least 2 arguments");
    return;
}

    for (index, argument) in env::args().enumerate() {
        println!("Argument index {index}, with argument {argument}");
        let arg2 = env::args().nth(2).unwrap();
        println!("Second argument is {arg2}");
    }
    
}
