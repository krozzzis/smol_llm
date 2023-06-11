use std::collections::HashMap;

use crate::vocab::Vocabulary;

/// Token identifier
pub type TokenId = u32;

pub trait Tokenizer {
    fn new() -> Self;

    /// Fills [`Vocabulary`] using [`TokenId`]s from given corpus
    fn fill_vocab(&self, corpus: &str, vocab: &mut Vocabulary);

    /// Converts given text to sequence of [`TokenId`]
    fn tokenize(&self, text: &str, vocab: &Vocabulary) -> Vec<TokenId>;
}

/// Character based BPE tokenizer
pub struct BpeTokenizer {
    pub max_size: usize,
}

impl Tokenizer for BpeTokenizer {
    fn new() -> Self {
        Self {
            max_size: 100,
        }
    }

    fn fill_vocab(&self, corpus: &str, vocab: &mut Vocabulary) {
        let mut corpus: Vec<TokenId> = corpus.chars().map(|ch| {
            if let Some(id) = vocab.try_add_token(ch.to_string()) {
                id
            } else {
                vocab.get_token_id(&ch.to_string()).unwrap()
            }
        }).collect();

        let mut freqs: HashMap<(TokenId, TokenId), u32> = HashMap::new();
        let len = self.max_size - vocab.len();
        for _ in 0..len {
            let mut flag = false;
            {
                for pair in corpus.windows(2) {
                    let pair = (pair[0], pair[1]);
                    if let Some(x) = freqs.get_mut(&pair) {
                        *x += 1;
                        flag = true;
                    } else {
                        freqs.insert(pair, 1);
                    }
                };
            }

            if !flag {
                break;
            }

            if let Some((max, _)) = freqs.iter().max_by_key(|x| x.1) {
                let token = vocab.get_token(max.0).unwrap().to_string() + vocab.get_token(max.1).unwrap();
                let token_id = vocab.try_add_token(token).unwrap();

                let mut i = 0;
                while i < corpus.len()-1 {
                    if (corpus[i], corpus[i+1]) == *max {
                        corpus.remove(i+1);
                        corpus.remove(i);
                        corpus.insert(i, token_id);
                    }
                    i += 1;
                }
            }
            freqs.clear();
        }
    }

    fn tokenize(&self, text: &str, vocab: &Vocabulary) -> Vec<TokenId> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bpe_fill_vocab() {
        let tokenizer = BpeTokenizer::new();
        let mut vocab = Vocabulary::new();

        tokenizer.fill_vocab("abcdedcba", &mut vocab);
        assert!(vocab.contains_token("a"));
        assert!(vocab.contains_token("b"));
        assert!(vocab.contains_token("c"));
        assert!(vocab.contains_token("d"));
        assert!(vocab.contains_token("e"));
        assert!(!vocab.contains_token("f"));
    }
}
