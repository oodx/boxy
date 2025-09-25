use boxy::api::layout::HeaderBuilder;

fn main() {
    let header = HeaderBuilder::new("This is a very long header text that definitely needs truncation")
        .build_for_width(15);
    
    println!("Header content: '{}'", header.content);
    println!("Header length: {}", header.content.len());
}
