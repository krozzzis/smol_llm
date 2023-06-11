pub mod vocab;
pub mod tokenizers;

use vocab::*;
use tokenizers::*;

fn main() {
    let mut vocab: Vocabulary = VocabularyBuilder::new()
        .add_token("[UNK]".to_string(), 0)
        .build();

    let corpus = std::fs::read_to_string("./input.txt").expect("Can't read file 'input.txt'");

    let mut tokenizer = tokenizers::BpeTokenizer::new();
    tokenizer.max_size = 1000;
    tokenizer.fill_vocab(&corpus, &mut vocab);

    println!("Vocabulary: ");
    println!("{}", vocab);

    let text = "World War II changed the political alignment and social structure of the globe";
    println!("Input: {}", text);
    let tokens = tokenizer.tokenize(text, &vocab);
    println!("TokenIds: {:?}", tokens);
    println!("Tokens: {:?}", tokens.iter().map(|id| vocab.get_token(*id).unwrap()).collect::<Vec<_>>());
}
