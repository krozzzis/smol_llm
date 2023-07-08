mod weighted_list;

pub use weighted_list::*;

use std::collections::HashMap;

use rand::{rngs::ThreadRng, Rng};
use tokenizer::tokenizers::*;

/// Simple random based language model which next token
/// prediction based only on one previous token and probabilities of
/// next tokens
pub struct MarkovChain {
    pub rand: ThreadRng,
    pub chain: HashMap<TokenId, WeightedVec<TokenId>>,
}

impl MarkovChain {
    pub fn new() -> Self {
        Self {
            rand: rand::thread_rng(),
            chain: HashMap::new(),
        }
    }

    /// Creates token-to-token-set distribution based on 
    /// given sequence of `TokenId`
    pub fn train_chain(&mut self, corpus: &Vec<TokenId>) {
        // Token -> (Token -> Count)
        let mut sets: HashMap<TokenId, HashMap<TokenId, usize>> = HashMap::new();
           
        for window in corpus[..].windows(2) {
            let token = window[0];
            let next = window[1];

            if let Some(set) = sets.get_mut(&token) {
                if let Some(cnt) = set.get_mut(&next) {
                    *cnt += 1;
                } else {
                    set.insert(next, 1);
                }
            } else {
                let mut set = HashMap::new();
                set.insert(next, 1);
                sets.insert(token, set);
            }
        }
        for (token, set) in sets.iter() {
            let mut a: WeightedVec<TokenId> = WeightedVec::new();
            for (el, cnt) in set.iter() {
                a.push(*cnt, *el);
            }
            self.chain.insert(*token, a);
        }
    }

    /// Generates next token depending on previous `TokenId`
    pub fn next_token(&mut self, token: TokenId) -> Option<TokenId> {
        if let Some(set) = self.chain.get(&token) {
            let size = set.next;
            let index = self.rand.gen_range(0..size);
            set.choose(index).cloned()
        } else {
            None
        }
    }
}
