pub mod token;
pub mod bpe;

use token::*;
use bpe::*;

fn main() {
    let mut table: TokenTable = TokenTable::new();
    let text = std::fs::read_to_string("./input.txt").expect("Can't read file 'input.txt'");

    let content = string_to_primary_tokens(&text, &mut table);
    fill_token_table(&content, &mut table);

    let mut tokens: Vec<_> = table.decode.keys().collect();
    tokens.sort();

    for token in tokens {
        println!("{} = \"{}\"", token, decode_token(*token, &table));
    }
}
