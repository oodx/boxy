fn main() {
    let info_plain = '\u{2139}'; // ℹ without selector
    println!("ℹ (U+2139) falls in range:");
    
    if info_plain >= '\u{2600}' && info_plain <= '\u{26FF}' {
        println!("  Miscellaneous Symbols - would be width 2");
    } else {
        println!("  Not in emoji ranges - would be width 1");
    }
}
