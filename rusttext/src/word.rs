use std::cmp::Ordering;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
pub enum EntryType {
    Word,
    Label,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WordEntry {
    pub word: String,
    pub entry_type: EntryType,
    pub count: u32,
    pub subwords: Vec<u32>,
}

impl WordEntry {
    pub fn new(word: &String, label_prefix: &String) -> WordEntry {
        let entry_type = get_type(word, label_prefix);

        WordEntry {
            word: word.clone(),
            count: 1,
            entry_type,
            subwords: Vec::new(),
        }
    }

    fn parse_subwords(&mut self, min_n: usize, max_n: usize) -> Vec<String> {
        if (min_n == 0) | (max_n == 0) {
            return Vec::new();
        }
        if min_n >= max_n {
            panic!("invalid subword parameters")
        }

        let mut subwords: Vec<String> = Vec::new();

        for width in min_n..max_n + 1 {
            for (i, letter) in self.word.bytes().enumerate() {
                if (letter & 0xC0) == 0x80 {
                    continue;
                }

                let ceil = i + width;
                if ceil <= self.word.len() {
                    let slice = &self.word[i..ceil];
                    subwords.push(String::from(slice));
                }
            }
        }

        return subwords;
    }

    pub fn compute_subwords(&mut self, min_n: usize, max_n: usize, bucket: u32) {
        let subword_chars = self.parse_subwords(min_n, max_n);
        let mut hashed_subwords: Vec<u32> = Vec::new();

        for subword in subword_chars.iter() {
            hashed_subwords.push(fnv_hash(subword) % bucket);
        }
        self.subwords = hashed_subwords;
    }
}

fn get_type(word: &String, label_prefix: &String) -> EntryType {
    match word {
        word if word.starts_with(label_prefix) => EntryType::Label,
        _ => EntryType::Word,
    }
}

pub fn fnv_hash(word: &String) -> u32 {
    let mut h: u32 = 2166136261;
    for char in word.bytes() {
        h = h ^ u32::from(char);
        h = h.wrapping_mul(16777619);
    }
    return h;
}

pub fn compare(left: &WordEntry, right: &WordEntry) -> Ordering {
    if left.entry_type == right.entry_type {
        right.count.cmp(&left.count)
    } else {
        left.entry_type.cmp(&right.entry_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hash() {
        assert_eq!(fnv_hash(&String::from("rust")), 490716647);
    }

    fn data_factory() -> [WordEntry; 3] {
        let word_0 = WordEntry {
            word: String::from("test_0"),
            count: 1,
            entry_type: EntryType::Word,
            subwords: Vec::new(),
        };
        let word_1 = WordEntry {
            word: String::from("test_1"),
            count: 2,
            entry_type: EntryType::Word,
            subwords: Vec::new(),
        };
        let label_0 = WordEntry {
            word: String::from("__label__test"),
            count: 1,
            entry_type: EntryType::Label,
            subwords: Vec::new(),
        };

        return [label_0, word_0, word_1];
    }

    #[test]
    fn test_get_entry_type() {
        let test_word = String::from("foo");
        let test_label = String::from("__label__foo");
        let label_prefix = String::from("__label__");

        assert_eq!(get_type(&test_label, &label_prefix), EntryType::Label);
        assert_eq!(get_type(&test_word, &label_prefix), EntryType::Word);
    }

    #[test]
    fn test_compare_words_before_labels() {
        let [test_label, test_word, _] = data_factory();
        assert_eq!(compare(&test_word, &test_label), Ordering::Less);
        assert_eq!(compare(&test_label, &test_word), Ordering::Greater);
    }

    #[test]
    fn test_compare_by_count() {
        let [_, test_word, test_more_word] = data_factory();
        assert_eq!(compare(&test_word, &test_more_word), Ordering::Greater);
    }

    #[test]
    fn test_sort_comparator() {
        let mut words = data_factory();
        let [test_label, test_word, test_more_word] = words.clone();
        let sorted_words = [test_more_word, test_word, test_label];

        words.sort_by(compare);

        assert_eq!(words, sorted_words);
    }

    #[test]
    fn test_subwords() {
        let label_prefix = String::from("__label__");
        let mut test_word = WordEntry::new(&String::from("rust"), &label_prefix);

        let subwords = test_word.parse_subwords(2, 3);
        let expected_subwords = ["ru", "us", "st", "rus", "ust"];
        assert_eq!(subwords, expected_subwords)
    }

    #[test]
    fn test_subwords_zero_param() {
        let label_prefix = String::from("__label__");
        let mut test_word = WordEntry::new(&String::from("rust"), &label_prefix);
        let empty: Vec<String> = Vec::new();

        assert_eq!(test_word.parse_subwords(0, 3), empty);
        assert_eq!(test_word.parse_subwords(2, 0), empty);
    }

    #[test]
    #[should_panic]
    fn test_subwords_bad_param() {
        let label_prefix = String::from("__label__");
        let mut test_word = WordEntry::new(&String::from("rust"), &label_prefix);

        test_word.parse_subwords(2, 1);
    }

    #[test]
    fn test_hashed_subwords() {
        let label_prefix = String::from("__label__");
        let mut test_word = WordEntry::new(&String::from("rust"), &label_prefix);
        let expected_hashes = [0, 9, 2, 7, 7];

        test_word.compute_subwords(2, 3, 10);
        assert_eq!(test_word.subwords, expected_hashes);
    }
}
