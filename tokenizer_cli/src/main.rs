use std::{path::PathBuf, str::FromStr};

use tokenizer::*;

fn print_usage() {
    let program = std::env::args().next().unwrap();
    println!("usage: {program} <subprogram>");
    println!("Subprograms: ");
    println!("  fill <vocab_file> <corpus_file>");
    println!("  tokenize <vocab_file> <text>");
}

fn fill_subcommand(vocab_path: PathBuf, corpus_path: PathBuf) -> Result<(), std::io::Error> {
    let mut vocab: Vocabulary = VocabularyBuilder::new()
        .add_token("[UNK]".to_string(), 0)
        .build();

    let corpus = std::fs::read_to_string(corpus_path.clone())?;

    let mut tokenizer = tokenizers::BpeTokenizer::new();
    tokenizer.max_size = 2000;
    tokenizer.fill_vocab(&corpus, &mut vocab);
    std::fs::write(vocab_path, vocab.serialize())?;

    Ok(())
}

fn tokenize_subcommand(vocab_path: PathBuf, text: &str) -> Result<String, std::io::Error> {
    let vocab = Vocabulary::from_str(&std::fs::read_to_string(vocab_path)?);
    let tokenizer = tokenizers::BpeTokenizer::new();
    let tokens = tokenizer.tokenize(text, &vocab);
    Ok(tokens.iter().map(|x| x.to_string()).collect::<Vec<_>>().as_slice().join(" "))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "fill" => {
            let vocab_path = if let Some(path) = args.get(2) {
                if let Some(x) = PathBuf::from_str(path).ok() {
                    x
                } else {
                    eprintln!("Can't read file {}", path);
                    print_usage();
                    return
                }
            } else {
                eprintln!("No vocab file");
                return
            };

            let corpus_path = if let Some(path) = args.get(3) {
                if let Some(x) = PathBuf::from_str(path).ok() {
                    x
                } else {
                    eprintln!("Can't read file {}", path);
                    print_usage();
                    return
                }
            } else {
                eprintln!("No corpus file");
                return
            };

            if let Err(e) = fill_subcommand(vocab_path, corpus_path) {
                eprintln!("{e}");
            }
        },
        "tokenize" => {
            let vocab_path = if let Some(path) = args.get(2) {
                if let Some(x) = PathBuf::from_str(path).ok() {
                    x
                } else {
                    eprintln!("Can't read file {}", path);
                    print_usage();
                    return
                }
            } else {
                eprintln!("No vocab file");
                return
            };

            if args.len() < 4 {
                eprintln!("No text for tokenization");
                print_usage();
                return;
            }
            let text = std::env::args().skip(3).collect::<Vec<_>>().join(" ");

            match tokenize_subcommand(vocab_path, &text) {
                Ok(tokens) => println!("{tokens}"),
                Err(e) => eprintln!("{e}"),
            } 
        },
        x => {
            eprintln!("Unknown command {x}");
            print_usage();
            return;
        }
    }
}
