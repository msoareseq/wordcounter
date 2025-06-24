use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    // Start time
    let start = Instant::now();
    
    
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

    let files = if show_top_n_words {
        args[2..].to_vec()
    } else {
        args[1..].to_vec()
    };

    // Setting global variables
    let mut total_words = 0;
    let mut words: HashMap<String, i32> = HashMap::new();

    // Reading files
    println!();
    println!("Reading files...");
    
    for file_name in files {
        let mut file_words = 0;
        let file = File::open(&file_name).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            match line {
                Ok(line) => {
                    for word in line
                        .replace(",", " ")
                        .replace(".", " ")
                        .replace(":", " ")
                        .replace("-", " ")
                        .split_whitespace()
                    {
                        file_words += 1;
                        total_words += 1;
                        words
                            .entry(word.to_lowercase())
                            .and_modify(|value| *value += 1)
                            .or_insert(1);
                    }
                }
                Err(_) => {}
            }
        }

        println!("File: {} - Words: {}", file_name, file_words);
    }
    println!("Total words: {}", total_words);
    println!("------------------------------");
    println!("Top-{} words", &n_words);
    println!();

    // Sorting top words
    let mut items: Vec<_> = words.iter().map(|(k, &v)| (k.clone(), v)).collect();
    items.sort_by(|a, b| b.1.cmp(&a.1));
    let top_n = items.into_iter().take(n_words as usize);

    // print top-n
    for (word, count) in top_n {
        println!("{:>15}: {:>5}", word, count);
    }
    println!();
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    println!();
}