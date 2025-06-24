use std::env;
use std::num::ParseIntError;

fn main() {
    
    // Argument handling
    let args: Vec<String> = env::args().collect();
    
    // Instructions
    let usage = r#"Usage: 
        wordcounter [n] <file1> <file2> ...
        
        Parameters:
            [n] - number of top n words to print (optional)
            <file> - files to read
    
    "#;
    
    
    if args.len() <= 1 {
        println!("{}", usage);
        return;
    }
    
    let mut show_top_n_words = false;
    let mut files: Vec<String> = vec![];
    
    let mut n_words = 10;
    
    match args[1].parse::<i32>() {
        Ok(n) => {
            n_words = n;
            show_top_n_words = true;
        }
        Err(_) => {
            println!("{}", "Top-n not defined. Showing top-10 words.");
        }
    }
    
    if show_top_n_words {
        files = args[2..].to_vec();
    } else { 
        files = args[1..].to_vec();
    }
    
}
