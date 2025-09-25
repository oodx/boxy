// Example demonstrating ANSI size overhead calculation
// Shows how much extra space ANSI escape codes add to strings

use boxy::api::geometry::{calculate_ansi_overhead, compare_ansi_sizes};

fn main() {
    println!("=== ANSI Size Comparison Example ===\n");

    // Example 1: Simple colored text
    let plain = "Hello, World!";
    let colored = "\x1b[32mHello, World!\x1b[0m";

    println!("Example 1: Simple colored text");
    println!("Plain text: \"{}\"", plain);
    println!("With color: \"{}\"", colored);

    let (total, display, overhead) = calculate_ansi_overhead(colored);
    println!("  Total bytes: {}", total);
    println!("  Display bytes: {}", display);
    println!("  ANSI overhead: {} bytes\n", overhead);

    // Example 2: Multiple colors
    let rainbow = "\x1b[31mR\x1b[33ma\x1b[32mi\x1b[36mn\x1b[34mb\x1b[35mo\x1b[31mw\x1b[0m";
    println!("Example 2: Rainbow text");
    println!("Colored: {}", rainbow);

    let (total, display, overhead) = calculate_ansi_overhead(rainbow);
    println!("  Total bytes: {}", total);
    println!("  Display bytes: {} (just 'Rainbow')", display);
    println!("  ANSI overhead: {} bytes", overhead);
    println!("  That's {}x the actual text!\n", total / display);

    // Example 3: Comparison with percentage
    let plain_log = "ERROR: Database connection failed";
    let colored_log = "\x1b[1;31mERROR\x1b[0m: Database connection failed";

    println!("Example 3: Log message comparison");
    let comparison = compare_ansi_sizes(plain_log, colored_log);

    println!("Plain version: \"{}\"", plain_log);
    println!("Styled version: {}", colored_log);
    println!("  Plain size: {} bytes", comparison.plain_bytes);
    println!("  Colored size: {} bytes", comparison.colored_bytes);
    println!("  Color overhead: {} bytes", comparison.color_overhead);
    println!("  Overhead percentage: {:.1}%\n", comparison.overhead_percentage);

    // Example 4: Heavy formatting
    let heavy = "\x1b[1;4;31;47mIMPORTANT\x1b[0m \x1b[33mWarning:\x1b[0m \x1b[1mSystem\x1b[0m \x1b[4mCritical\x1b[0m";
    println!("Example 4: Heavy formatting");
    println!("Formatted: {}", heavy);

    let (total, display, overhead) = calculate_ansi_overhead(heavy);
    println!("  Total bytes: {}", total);
    println!("  Display bytes: {}", display);
    println!("  ANSI overhead: {} bytes", overhead);
    println!("  Storage efficiency: {:.1}%\n", (display as f64 / total as f64) * 100.0);

    // Example 5: Practical implications
    println!("=== Practical Implications ===\n");

    let log_line = "[2024-01-15 10:30:45] INFO: Request processed successfully";
    let colored_log_line = "\x1b[90m[2024-01-15 10:30:45]\x1b[0m \x1b[36mINFO\x1b[0m: Request processed successfully";

    let comparison = compare_ansi_sizes(log_line, colored_log_line);

    println!("For a typical log line:");
    println!("  Plain: {} bytes", comparison.plain_bytes);
    println!("  Colored: {} bytes", comparison.colored_bytes);
    println!("  Overhead: {} bytes ({:.1}%)", comparison.color_overhead, comparison.overhead_percentage);

    // Calculate impact at scale
    let logs_per_day = 100_000;
    let daily_overhead_mb = (comparison.color_overhead * logs_per_day) as f64 / (1024.0 * 1024.0);
    let yearly_overhead_gb = daily_overhead_mb * 365.0 / 1024.0;

    println!("\nAt {} logs/day:", logs_per_day);
    println!("  Daily overhead: {:.2} MB", daily_overhead_mb);
    println!("  Yearly overhead: {:.2} GB", yearly_overhead_gb);
    println!("\nThis is why many systems strip ANSI codes before storage!");

    // Example 6: Network transmission
    println!("\n=== Network Transmission Impact ===\n");

    let json_plain = r#"{"status":"success","message":"Operation completed"}"#;
    let json_colored = r#"{"status":"\x1b[32msuccess\x1b[0m","message":"Operation completed"}"#;

    let comparison = compare_ansi_sizes(json_plain, json_colored);

    println!("JSON response:");
    println!("  Without ANSI: {} bytes", comparison.plain_bytes);
    println!("  With ANSI: {} bytes", comparison.colored_bytes);
    println!("  Overhead: {} bytes ({:.1}%)", comparison.color_overhead, comparison.overhead_percentage);

    let requests_per_second = 1000;
    let bandwidth_overhead_mbps = (comparison.color_overhead * requests_per_second * 8) as f64 / (1024.0 * 1024.0);

    println!("\nAt {} requests/second:", requests_per_second);
    println!("  Extra bandwidth: {:.3} Mbps", bandwidth_overhead_mbps);
    println!("  That's bandwidth you're paying for but not using for data!");
}