use anyhow::Result;

/// Main tokenization function that takes a string and returns a vector of tokenized strings
pub fn tokenize_text(text: &str) -> Result<Vec<String>> {
    // Simple sentence-based chunking without the tokenizers crate
    Ok(text.split(&['.', '!', '?'])
        .filter(|s| !s.trim().is_empty())
        .filter(|s| s.split_whitespace().count() >= 5)
        .map(|s| s.trim().to_string())
        .collect())
}