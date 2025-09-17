fn main() {
    let symbols = vec!["🟢", "🟠", "🔴", "🌐", "📦", "ℹ️"];
    for s in symbols {
        println!("{}: U+{:04X}", s, s.chars().next().unwrap() as u32);
        for ch in s.chars() {
            println!("  char: U+{:04X}", ch as u32);
        }
    }
}
