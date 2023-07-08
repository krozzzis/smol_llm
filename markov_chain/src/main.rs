use std::{fs, io::{stdin, stdout, Write}, path::PathBuf, str::FromStr, time::Instant};

use markov_chain::MarkovChain;
use tokenizer::{Vocabulary, tokenizers::BpeTokenizer, tokenizers::Tokenizer};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let vocab_path = if let Some(path) = args.get(1) {
        if let Some(x) = PathBuf::from_str(path).ok() {
            x
        } else {
            eprintln!("Can't read file {}", path);
            return
        }
    } else {
        eprintln!("No vocab file");
        return
    };

    let corpus_path = if let Some(path) = args.get(2) {
        if let Some(x) = PathBuf::from_str(path).ok() {
            x
        } else {
            eprintln!("Can't read file {}", path);
            return
        }
    } else {
        eprintln!("No corpus file");
        return
    };

    let vocab = Vocabulary::from_str(&std::fs::read_to_string(vocab_path).unwrap());
    
    println!("Tokenizing corpus...");
    let bpe = BpeTokenizer::new();
    let corpus = fs::read_to_string(corpus_path).unwrap();
    let tokens = bpe.tokenize(&corpus, &vocab);

    println!("Training chain...");
    let mut chain = MarkovChain::new();
    stdout().flush().unwrap();
    chain.train_chain(&tokens);

    println!("Ready!\n");
    let mut buffer = String::new();
    loop {
        buffer.clear();
        if let Err(e) = stdin().read_line(&mut buffer) {
            eprintln!("{e}");
            continue;
        }
        buffer = buffer.replace('\n', "");
        let tokens = bpe.tokenize(&buffer, &vocab);
        let mut prev = tokens[tokens.len()-1];
        print!("\x1b[1A{}", &buffer);
        stdout().flush().unwrap();

        let mut time = Instant::now();

        for _ in 0..256 {
            let id = match chain.next_token(prev) {
                Some(id) => id,
                None => break,
            };
            prev = id;
            let token = vocab.get_token(id).unwrap();
            while time.elapsed().as_millis() < 50 { }

            print!("{}", token);
            print!("\x1b[0m");
            stdout().flush().unwrap();
            time = Instant::now();
        }
        print!("\n\n");
        stdout().flush().unwrap();
    }
}
