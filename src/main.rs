use std::env;
use std::fs;
use std::io;
use std::collections::HashMap;


fn main() {

if env::args().len() <= 1 {
    println!("We need at least 1 argument as a filename");
    return;
}

    let arg = env::args().nth(1);
    let arg1 = match arg {
        Some(message) => message,
        None => {
            eprintln!("There are no arguments");
            std::process::exit(2);
        }
    };
    let result = fs::read_to_string(arg1);
    let contents = match result {
        Ok(message) => message,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => {
            eprintln!("Could not read file: {}", error);
            std::process::exit(1);
            }

            io::ErrorKind::PermissionDenied => {
                eprintln!("Permission denied to access file: {}", error);
                std::process::exit(1);
            }
            _ => {
                eprintln!("Another kind of error accessing file: {}", error);
                std::process::exit(1);
            }
        }
    };
    let mut wordmap = HashMap::new();
    for word in str::split_whitespace(&contents) {
        //println!("The current word is {word}");
        wordmap.entry(str::to_lowercase(word))
        .and_modify(|e| { *e += 1 })
        .or_insert(1);
    }
    //println!("The hasmap is {:?}", wordmap);
    let mut mostcommonmap = HashMap::new();
    let mut largest = 0;
    for (_k, v) in &wordmap {
        if v > &largest {
            largest = *v;
        };
    }

    for (k, v) in &wordmap {
        if v == &largest {
            mostcommonmap.insert(k,v);
        }
            
    };
    


    //println!("The hasmap is {:?}", wordmap);
    println!("The most common word appeared {largest} times");
    println!("The most common words are {:?}", mostcommonmap);

}

