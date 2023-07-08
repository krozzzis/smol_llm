# Smol Language Model

It's like Large Language Model, but smol.

I don't use any dependencies in this projects(except for Rand). 
All subprojects will made from scratch and only in Rust.

## Roadmap
- [x] BPE Tokenizer
- [ ] Interactive tokenizer and vocab viewer in HTML/WASM
- [x] Simple language model(e.g. using Markov chain)
- [ ] Telegram bot and web app for models
- [ ] Simple neural network implementation
- [ ] Neural network training/inference framework
- [ ] Word embedding, word2vec
- [ ] Interactive word similarity viewer in HTML/WASM
- [ ] Generative model for text

## Usage

### Tokenizer

```bash
$ cargo run --bin tokenizer_cli
```

### Markov chain

Interactive mode:
```bash
$ cargo run --bin markov_chain -- content/vocab.vcb content/corpus.txt
```
