use std::collections::HashMap;

use crate::tokenizers::TokenId;

/// Contains token-value pairs
#[derive(Debug, Clone)]
pub struct Vocabulary {
    decode: HashMap<TokenId, String>,
    encode: HashMap<String, TokenId>,
    next_token: TokenId,
}

impl Vocabulary {
    pub fn new() -> Self {
        Self {
            decode: HashMap::new(),
            encode: HashMap::new(),
            next_token: 0,
        }
    }

    /// If token is unknown adds token to vocabulary and returns its [`TokenId`],
    /// if token already known returns `None`
    pub fn try_add_token(&mut self, value: String) -> Option<TokenId> {
        if self.encode.contains_key(value.as_str()) {
            None
        } else {
            let token = self.next_token;
            self.add_token(value, token);
            Some(token)
        }
    }

    /// Adds token to vocabulary with given [`TokenId`]
    pub fn add_token(&mut self, token: String, id: TokenId) {
        self.encode.insert(token.clone(), id);
        self.decode.insert(id, token);

        while self.decode.contains_key(&self.next_token) {
            self.next_token += 1;
        }
    }

    /// Returns `true` if vocabulary contains token
    pub fn contains_token(&self, token: &str) -> bool {
        self.encode.contains_key(&token.to_string())
    }

    /// Returns `true` if vocabulary contains [`TokenId`]
    pub fn contains_token_id(&self, id: TokenId) -> bool {
        self.decode.contains_key(&id)
    }

    /// Returns [`TokenId`] if token is in vocabulary,
    /// otherwise returns `None`
    pub fn get_token_id(&self, token: &str) -> Option<TokenId> {
        self.encode.get(token).cloned()
    }

    /// Returns token if [`TokenId`] is in vocabulary,
    /// otherwise returns `None`
    pub fn get_token(&self, id: TokenId) -> Option<&str> {
        self.decode.get(&id).map(|x| x.as_str())
    }

    /// Returns vocabulary lenght
    pub fn len(&self) -> usize {
        self.decode.len()
    }
}

impl std::fmt::Display for Vocabulary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tokens: Vec<(TokenId, &str)> = self.decode.iter().map(|(a, x)| (*a, x.as_str())).collect();        
        tokens.sort_by_key(|x| x.0);
        for (id, token) in tokens {
            write!(f, "{} = \"{}\"\n", id, token)?;
        }
        Ok(())
    }
}

pub struct VocabularyBuilder {
    vocab: Vocabulary,
}

impl VocabularyBuilder {
    pub fn new() -> Self {
        Self {
            vocab: Vocabulary::new(),
        }
    }

    pub fn build(self) -> Vocabulary {
        self.vocab
    }

    pub fn try_add_token(mut self, token: String) -> Self {
        self.vocab.try_add_token(token);
        self
    }

    pub fn add_token(mut self, token: String, id: TokenId) -> Self {
        self.vocab.add_token(token, id);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_add_token() {
        let mut vocab = Vocabulary::new();
        assert_eq!(vocab.try_add_token("hellow".to_string()), Some(0));
        assert_eq!(vocab.try_add_token("hellow".to_string()), None);
        assert_eq!(vocab.get_token_id("hellow"), Some(0));

        assert_eq!(vocab.try_add_token("aboba".to_string()), Some(1));
        assert_eq!(vocab.try_add_token("aboba".to_string()), None);
        assert_eq!(vocab.get_token_id("aboba"), Some(1));

        assert_eq!(vocab.try_add_token("achtung".to_string()), Some(2));
        assert_eq!(vocab.try_add_token("achtung".to_string()), None);
        assert_eq!(vocab.get_token_id("achtung"), Some(2));

        assert_eq!(vocab.get_token_id("world"), None);
    }

    #[test]
    fn add_token() {
        let mut vocab = Vocabulary::new();
        vocab.add_token("hellow".to_string(), 0);
        assert_eq!(vocab.get_token_id("hellow"), Some(0));

        vocab.add_token("hellow".to_string(), 0);
        assert_eq!(vocab.get_token_id("hellow"), Some(0));

        vocab.add_token("aboba".to_string(), 0);
        assert_eq!(vocab.get_token_id("aboba"), Some(0));

        vocab.add_token("hellow".to_string(), 1);
        assert_eq!(vocab.get_token_id("hellow"), Some(1));
    } 

    #[test]
    fn builder1() {
        let vocab = VocabularyBuilder::new()
            .try_add_token("hellow".to_string())
            .try_add_token("world".to_string())
            .try_add_token("aboba".to_string())
            .build();
        
        assert_eq!(vocab.get_token(0), Some("hellow"));
        assert_eq!(vocab.get_token(1), Some("world"));
        assert_eq!(vocab.get_token(2), Some("aboba"));
        assert_eq!(vocab.get_token(3), None);
    }

    #[test]
    fn builder2() {
        let vocab = VocabularyBuilder::new()
            .add_token("hellow".to_string(), 0)
            .add_token("world".to_string(), 2)
            .add_token("aboba".to_string(), 5)
            .try_add_token("lol".to_string())
            .try_add_token("foo".to_string())
            .build();
        
        assert_eq!(vocab.get_token(0), Some("hellow"));
        assert_eq!(vocab.get_token(2), Some("world"));
        assert_eq!(vocab.get_token(5), Some("aboba"));
        assert_eq!(vocab.get_token(1), Some("lol"));
        assert_eq!(vocab.get_token(3), Some("foo"));

        assert_eq!(vocab.get_token(6), None);
    }
}
