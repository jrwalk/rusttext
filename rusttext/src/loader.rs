use std::fs::File;
use std::io::{BufReader, Read, Write};

use crate::{vocabulary, word, Result};

pub fn read_from_iter<'a, I>(vocab: &mut vocabulary::Vocabulary, words: I)
where
    I: Iterator<Item = &'a String>,
{
    for word in words {
        vocab.add(word)
    }
}
