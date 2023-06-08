use std::collections::BTreeMap;

use crate::token::{TokenTable, Token, TokenValue};

/// Convert string to vector of simple tokens represening chars.
/// Adds unknown chars to table
pub fn string_to_primary_tokens(content: &str, table: &mut TokenTable) -> Vec<Token> {
    content.chars().map(move |x| {
        let value = TokenValue::Primary(x);
        if let Some(token) = table.get_token(value) {
            token
        } else {
            table.new_token(value)
        }
    }).collect()
}

/// Fills token table with pair tokens made using bpe
pub fn fill_token_table(content: &Vec<Token>, table: &mut TokenTable) {
    let mut pair_freqs: BTreeMap<(Token, Token), u32> = BTreeMap::new();
    let mut content = content.clone();

    'a: for _ in 0..200 {
        let mut flag = false;
        for pair in content.windows(2) {
            if let Some(x) = pair_freqs.get_mut(&(pair[0], pair[1])) {
                *x += 1;
                flag = true;
            } else {
                pair_freqs.insert((pair[0], pair[1]), 1);
            }
        }
        if !flag {
            break 'a;
        }
        if let Some((pair, _)) = pair_freqs.iter().max_by_key(|(_, b)| *b) {
            let token = table.new_token(TokenValue::Pair(pair.0, pair.1));
            let mut i = 0;
            while i < content.len()-1 {
                if (content[i], content[i+1]) == *pair {
                    content.remove(i+1);
                    content.remove(i);
                    content.insert(i, token);
                }
                i += 1;
            }
        } else {
            break 'a;
        }
        pair_freqs.clear();
    }
}
