use crate::word;
use std::iter::FromIterator;

pub struct Vocabulary {
    words: Vec<word::WordEntry>,
    word_to_index: Vec<i32>,
    vocab_size: usize,
    n_tokens: u32,
    n_words: u32,
    n_labels: u32,
    size: u32,
    label_prefix: String,
    min_n: usize,
    max_n: usize,
    bucket: u32,
}

impl Vocabulary {
    pub fn new(vocab_size: usize, min_n: usize, max_n: usize, bucket: u32) -> Vocabulary {
        Vocabulary {
            words: Vec::new(),
            word_to_index: vec![-1; vocab_size],
            vocab_size,
            n_tokens: 0,
            n_words: 0,
            n_labels: 0,
            size: 0,
            label_prefix: String::from("__label__"),
            min_n,
            max_n,
            bucket,
        }
    }

    fn hash_lookup(&self, word: &String) -> usize {
        let mut word_hash = word::fnv_hash(&word) as usize % self.vocab_size;
        let mut word_index = self.word_to_index[word_hash];
        loop {
            match (word_index, word) {
                (-1, _) => return word_hash,
                (_, word) if word == &self.words[word_index as usize].word => return word_hash,
                _ => {
                    word_hash = (word_hash + 1) % self.vocab_size;
                    word_index = self.word_to_index[word_hash];
                }
            }
        }
    }

    fn get_id(&self, word: &String) -> i32 {
        let hash = self.hash_lookup(word);
        return self.word_to_index[hash];
    }

    pub fn add(&mut self, word: &String) {
        let hash = self.hash_lookup(word);
        let index = self.word_to_index[hash];
        self.n_tokens += 1;

        match index {
            -1 => {
                let mut word_entry = word::WordEntry::new(word, &self.label_prefix);
                if word_entry.entry_type == word::EntryType::Word {
                    word_entry.compute_subwords(self.min_n, self.max_n, self.bucket);
                }
                self.words.push(word_entry);
                self.word_to_index[hash] = self.size as i32;
                self.size += 1;
            }
            _ => {
                self.words[index as usize].count += 1;
            }
        }
    }

    fn threshold(&mut self, word_threshold: u32, label_threshold: u32) {
        // prune words below threshold
        self.words.sort_by(word::compare);

        self.words.retain(|word| match word.entry_type {
            word::EntryType::Word => word.count >= word_threshold,
            word::EntryType::Label => word.count >= label_threshold,
        });

        // reset counters
        self.size = 0;
        self.n_words = 0;
        self.n_labels = 0;
        self.word_to_index = vec![-1; self.vocab_size];

        // re-hydrate lookup
        for word in self.words.iter() {
            let hash = self.hash_lookup(&word.word);
            self.word_to_index[hash] = self.size as i32;
            self.size += 1;
            match word.entry_type {
                word::EntryType::Word => self.n_words += 1,
                word::EntryType::Label => self.n_labels += 1,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_vocab() -> Vocabulary {
        let label_prefix = String::from("__label__");
        let foo = word::WordEntry::new(&String::from("foo"), &label_prefix);
        let bar = word::WordEntry::new(&String::from("bar"), &label_prefix);
        let baz = word::WordEntry::new(&String::from("__label__baz"), &label_prefix);

        Vocabulary {
            words: vec![foo, bar, baz],
            n_tokens: 3,
            n_words: 2,
            n_labels: 1,
            size: 3,
            label_prefix,
            vocab_size: 5,
            word_to_index: vec![-1, 2, 1, 0, -1],
            min_n: 2,
            max_n: 4,
            bucket: 10,
        }
    }

    #[test]
    fn test_hash_lookup() {
        let test_vocab = test_vocab();

        assert_eq!(test_vocab.hash_lookup(&String::from("foo")), 3);
        assert_eq!(test_vocab.hash_lookup(&String::from("bar")), 2);
        assert_eq!(test_vocab.hash_lookup(&String::from("__label__baz")), 1);
    }

    #[test]
    fn test_get_id() {
        let test_vocab = test_vocab();

        assert_eq!(test_vocab.get_id(&String::from("foo")), 0);
        assert_eq!(test_vocab.get_id(&String::from("bar")), 1);
        assert_eq!(test_vocab.get_id(&String::from("__label__baz")), 2);
    }

    #[test]
    fn test_add() {
        let mut test_vocab = test_vocab();
        let test_word = String::from("biff");

        test_vocab.add(&test_word);

        assert_eq!(test_vocab.hash_lookup(&test_word), 0);
        assert_eq!(test_vocab.get_id(&test_word), 3);
        assert_eq!(test_vocab.n_tokens, 4);
    }
}
