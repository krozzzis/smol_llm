use std::collections::HashMap;

/// Token identifier
pub type Token = u32;

/// Represent value associated with Token identifier
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum TokenValue {
    Primary(char),
    Pair(Token, Token),
}

impl std::fmt::Debug for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenValue::Primary(x) => write!(f, "\"{}\"", x),
            TokenValue::Pair(a, b) => write!(f, "{} + {}", a, b),
        }
    }
}

/// Dictionary where each token identifier matched with token value
#[derive(Debug, Clone)]
pub struct TokenTable {
    pub encode: HashMap<TokenValue, Token>,
    pub decode: HashMap<Token, TokenValue>,
    pub next_token: Token,
}

impl TokenTable {
    pub fn new() -> Self {
        Self {
            encode: HashMap::new(),
            decode: HashMap::new(),
            next_token: 0,
        }
    }

    pub fn get_value(&self, token: Token) -> Option<TokenValue> {
        self.decode.get(&token).cloned()
    }

    pub fn get_token(&self, value: TokenValue) -> Option<Token> {
        self.encode.get(&value).cloned()
    }

    pub fn contains_value(&self, value: TokenValue) -> bool {
        self.encode.contains_key(&value)
    }

    pub fn add_token(&mut self, value: TokenValue) {
        self.encode.insert(value, self.next_token);
        self.decode.insert(self.next_token, value);
    }

    pub fn new_token(&mut self, value: TokenValue) -> Token {
        self.encode.insert(value, self.next_token);
        self.decode.insert(self.next_token, value);

        let out = self.next_token;
        while self.decode.contains_key(&self.next_token) {
            self.next_token += 1;
        }

        out
    }
}

pub fn decode_token(token: Token, table: &TokenTable) -> String {
    if let Some(TokenValue::Primary(ch)) = table.get_value(token) {
        String::from(ch)
    } else if let Some(TokenValue::Pair(a, b)) = table.get_value(token) {
        let mut result = String::new();

        result.push_str(decode_token(a, table).as_str());
        result.push_str(decode_token(b, table).as_str());

        result
    } else {
        String::new()
    }
}
