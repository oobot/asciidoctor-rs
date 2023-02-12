use std::collections::{HashSet, LinkedList};

use anyhow::Error;
use async_std::fs;
use async_std::path::Path;
use once_cell::sync::Lazy;

pub static BLOCK_TOKENS: Lazy<HashSet<&str>> = Lazy::new(|| HashSet::from(
    ["|===",  "====", "----", "____", "****", "++++"]
));

pub async fn read_file(file: &Path) -> Result<LinkedList<String>, Error> {
    Ok(read(&fs::read_to_string(file).await?))
}

pub fn read(text: &str) -> LinkedList<String> {
    let mut list = LinkedList::new();
    let mut block = String::new();
    let mut combine_block = false;
    for line in text.split("\n\n") {
        block.push_str(line);
        block.push_str("\n\n");

        if !combine_block && only_one_token(line) {
            combine_block = true;
            continue;
        }

        if combine_block && only_one_token(line) && is_token_end(line) {
            combine_block = false;
        }

        if !combine_block {
            match list.is_empty() { // header
                true => list.append(&mut split_header(&block)),
                _ => list.push_back(block.trim().to_owned()),
            }
            block = String::new();
        }
    }

    // end with :: and +, combine in a block.
    list
}

fn split_header(header: &str) -> LinkedList<String> {
    header.split("\n").filter_map(|v|
        match v.trim().is_empty() {
            true => None,
            _ => Some(v.trim().to_owned())
        }
    ).collect()
}

fn only_one_token(block: &str) -> bool {
    let mut count = 0;
    for b in block.split("\n") {
        count += match BLOCK_TOKENS.contains(b.trim()) {
            true => 1,
            _ => 0
        };
    }
    count == 1
}

fn is_token_end(block: &str) -> bool {
    for token in BLOCK_TOKENS.iter() {
        if block.ends_with(token) {
            return true;
        }
    }
    false
}
