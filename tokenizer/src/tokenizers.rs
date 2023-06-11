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
        let mut text: Vec<String> = text.chars().map(|ch| ch.to_string()).collect();
        
        let mut i = 0;
        while i < text.len()-1 {
            let current = text[i].clone() + &text[i+1];

            if vocab.contains_token(&current) {
                text.remove(i+1);
                text.remove(i);
                text.insert(i, current)
            } else {
                i += 1;
            }
        }

        let unk_token = if let Some(id) = vocab.get_token_id("[UNK]") { id } else { 0 };
        text.iter().map(|ch| {
            if let Some(id) = vocab.get_token_id(&ch.to_string()) {
                id
            } else {
                unk_token
            }
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::vocab::VocabularyBuilder;

    use super::*;

    #[test]
    fn bpe_fill_vocab1() {
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

    #[test]
    fn bpe_fill_vocab2() {
        let tokenizer = BpeTokenizer::new();
        let mut vocab = Vocabulary::new();

        tokenizer.fill_vocab("ababcabab", &mut vocab);
        assert!(vocab.contains_token("a"));
        assert!(vocab.contains_token("b"));
        assert!(vocab.contains_token("c"));
        assert!(vocab.contains_token("ab"));
        assert!(vocab.contains_token("abab"));
        assert!(!vocab.contains_token("abc"));
        assert!(!vocab.contains_token("ababc"));
        assert!(!vocab.contains_token("ababc"));
    }

    #[test]
    fn bpe_tokenize1() {
        let tokenizer = BpeTokenizer::new();
        let vocab = VocabularyBuilder::new()
            .add_token("[UNK]".to_string(), 0)
            .add_token("a".to_string(), 1)
            .add_token("b".to_string(), 2)
            .add_token("ab".to_string(), 3)
            .add_token("bc".to_string(), 4)
            .build();

        let tokens = tokenizer.tokenize("abab", &vocab);
        assert_eq!(tokens, vec![3, 3]);

        let tokens = tokenizer.tokenize("!abab", &vocab);
        assert_eq!(tokens, vec![0, 3, 3]);

        let tokens = tokenizer.tokenize("abc", &vocab);
        assert_eq!(tokens, vec![3, 0]);
    }
}
