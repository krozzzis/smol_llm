pub mod vocab;
pub mod tokenizers;

use vocab::*;
use tokenizers::*;

fn main() {
    let mut vocab: Vocabulary = VocabularyBuilder::new()
        .add_token("[UNK]".to_string(), 0)
        .build();

    // let corpus = std::fs::read_to_string("./input.txt").expect("Can't read file 'input.txt'");
    let corpus = "Sed alias eos dolorem inventore. Tenetur consequatur repellendus nostrum unde minima laudantium incidunt perspiciatis. Ut voluptas occaecati eaque quo placeat occaecati provident. Repellendus eveniet magni sit adipisci. Repudiandae ipsa ea atque reiciendis quibusdam ut porro.";

    let tokenizer = tokenizers::BpeTokenizer::new();
    tokenizer.fill_vocab(&corpus, &mut vocab);

    println!("{}", vocab);

    // let tokens = tokenizer.tokenize("Hellow world!", &vocab);
    // println!("{:?}", tokens);
}
