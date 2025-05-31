
mod pdf_to_json;
mod tokenizer;
use anyhow::{Context, Result};



fn main() -> Result<()> {

    
    let pdf_path = "pdfs/WW Guide 3-25-1.pdf";
    let json_output = pdf_to_json::pdf_to_json_converter(pdf_path).context("Pdf coulsnt convert to Json")?;

    let text = json_output["extracted_text"].as_str().unwrap();
    let tokens = tokenizer::tokenize_text(text).context("Failed to tokenize")?;
    printer(&tokens);
    




    Ok(())
 
}


fn printer(tokens: &Vec<String>) {
    for (i, token) in tokens.iter().enumerate() {
        println!("Chunk {}: {}", i + 1, token);
        println!("Word count: {}\n", token.split_whitespace().count());
    }
    println!("Total chunks: {}", tokens.len());
}