use std::collections::{HashSet, LinkedList};

use anyhow::Error;
use async_std::fs;
use async_std::path::Path;
use once_cell::sync::Lazy;

pub async fn read_file(file: &Path) -> Result<LinkedList<String>, Error> {
    Ok(read(&fs::read_to_string(file).await?))
}

pub fn read(text: &str) -> LinkedList<String> {
    // let lines: Vec<&str> = text.split("\n").collect();
    let mut list = LinkedList::new();

    let mut block = String::new();
    let mut combine_block = false;
    for line in text.split("\n\n") {
        block.push_str(line);
        block.push_str("\n\n");

        if !combine_block && has_block_token(line) {
            combine_block = true;
            continue;
        }

        if combine_block && is_token_end(line) {
            combine_block = false;
        }

        if !combine_block {
            list.push_back(block);
            block = String::new();
        }
    }

    // end with :: and +, combine in a block.
    list
}

fn has_block_token(line: &str) -> bool {
    for l in line.split("\n") {
        if BLOCK_TOKENS.contains(l) {
            return true;
        }
    }
    false
}

fn is_token_end(line: &str) -> bool {
    for token in BLOCK_TOKENS.iter() {
        if line.ends_with(token) {
            return true;
        }
    }
    false
}

static BLOCK_TOKENS: Lazy<HashSet<&str>> = Lazy::new(|| HashSet::from(
    ["|===",  "====", "----", "____", "****", "++++"]
));
