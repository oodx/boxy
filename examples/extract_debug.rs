// Debug content extraction
use boxy::api::layout::{BoxBuilder, HeaderBuilder};

fn main() {
    println!("=== Content Extraction Debug ===\n");

    let layout = BoxBuilder::new("Content")
        .with_header(HeaderBuilder::new("Test Header"))
        .with_fixed_width(40)
        .with_barmode()
        .build();

    // Manual text extraction test
    let header_line = "┌─────────────Test Header──────────────┐";
    println!("Original header: '{}'", header_line);

    // Extract content manually
    if header_line.len() >= 2 {
        let chars: Vec<char> = header_line.chars().collect();
        if chars[0] == '┌' && chars[chars.len() - 1] == '┐' {
            let extracted: String = chars[1..chars.len() - 1].iter().collect();
            println!("Extracted content: '{}'", extracted);
            println!("Extracted length: {}", extracted.len());
            println!("Trimmed: '{}'", extracted.trim());
            println!("Trimmed length: {}", extracted.trim().len());
        }
    }
}
