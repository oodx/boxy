use unicode_width::UnicodeWidthStr;
use std::io::{self, Read};
use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

struct BoxStyle {
    top_left: &'static str,
    top_right: &'static str,
    bottom_left: &'static str,
    bottom_right: &'static str,
    horizontal: &'static str,
    vertical: &'static str,
}

const NORMAL: BoxStyle = BoxStyle {
    top_left: "┌", top_right: "┐",
    bottom_left: "└", bottom_right: "┘",
    horizontal: "─", vertical: "│",
};

const ROUNDED: BoxStyle = BoxStyle {
    top_left: "╭", top_right: "╮",
    bottom_left: "╰", bottom_right: "╯",
    horizontal: "─", vertical: "│",
};

const DOUBLE: BoxStyle = BoxStyle {
    top_left: "╔", top_right: "╗",
    bottom_left: "╚", bottom_right: "╝",
    horizontal: "═", vertical: "║",
};

const HEAVY: BoxStyle = BoxStyle {
    top_left: "┏", top_right: "┓",
    bottom_left: "┗", bottom_right: "┛",
    horizontal: "━", vertical: "┃",
};

const ASCII: BoxStyle = BoxStyle {
    top_left: "+", top_right: "+",
    bottom_left: "+", bottom_right: "+",
    horizontal: "-", vertical: "|",
};

fn get_color_code(color: &str) -> &'static str {
    match color {
        "red" => "\x1B[38;5;9m",
        "red2" => "\x1B[38;5;197m",
        "deep" => "\x1B[38;5;61m",
        "deep_green" => "\x1B[38;5;60m",
        "orange" => "\x1B[38;5;214m",
        "yellow" => "\x1B[33m",
        "green" => "\x1B[38;5;10m",
        "green2" => "\x1B[32m",
        "blue" => "\x1B[36m",
        "blue2" => "\x1B[38;5;39m",
        "cyan" => "\x1B[38;5;14m",
        "magenta" => "\x1B[35m",
        "purple" => "\x1B[38;5;213m",
        "purple2" => "\x1B[38;5;141m",
        "white" => "\x1B[38;5;247m",
        "white2" => "\x1B[38;5;15m",
        "grey" => "\x1B[38;5;242m",
        "grey2" => "\x1B[38;5;240m",
        "grey3" => "\x1B[38;5;237m",
        _ => "",
    }
}

const RESET: &str = "\x1B[0m";

fn get_display_width(text: &str) -> usize {
    let clean = strip_ansi_escapes::strip(text);
    let clean_str = String::from_utf8_lossy(&clean);
    UnicodeWidthStr::width(clean_str.as_ref())
}

fn draw_box(text: &str, padding: usize, style: &BoxStyle, color: &str) {
    let lines: Vec<&str> = text.lines().collect();
    
    let max_width = lines.iter()
        .map(|line| get_display_width(line))
        .max()
        .unwrap_or(0);
    
    let inner_width = max_width + 2 * padding;
    let border = style.horizontal.repeat(inner_width);
    let pad = " ".repeat(padding);
    
    let color_code = get_color_code(color);
    
    // Top border
    println!("{}{}{}{}{}", 
        color_code, style.top_left, border, style.top_right, RESET);
    
    // Content lines
    for line in lines {
        let width = get_display_width(line);
        let spaces = " ".repeat(max_width - width);
        println!("{}{}{}{}{}{}{}{}",
            color_code, style.vertical, RESET,
            pad, line, spaces, pad,
            format!("{}{}{}", color_code, style.vertical, RESET));
    }
    
    // Bottom border
    println!("{}{}{}{}{}", 
        color_code, style.bottom_left, border, style.bottom_right, RESET);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut style = &NORMAL;
    let mut color = "none";
    let mut skip_next = false;
    
    for (i, arg) in args.iter().enumerate().skip(1) {
        if skip_next {
            skip_next = false;
            continue;
        }
        
        match arg.as_str() {
            "--help" | "-h" => {
                println!("{} {} - {}", NAME, VERSION, DESCRIPTION);
                println!("\nUSAGE:");
                println!("    echo \"text\" | {} [OPTIONS]", NAME);
                println!("\nOPTIONS:");
                println!("    -s, --style <STYLE>  Box style [normal, rounded, double, heavy, ascii]");
                println!("    -c, --color <COLOR>  Box color [red, red2, green, green2, blue, blue2,");
                println!("                         cyan, orange, yellow, purple, purple2, magenta,");
                println!("                         deep, deep_green, white, white2, grey, grey2, grey3]");
                println!("    -h, --help           Show this help message");
                println!("    -V, --version        Show version");
                println!("\nEXAMPLES:");
                println!("    echo \"Hello\" | {}", NAME);
                println!("    echo \"Hello\" | {} --style rounded --color blue", NAME);
                println!("    echo \"Hello\" | {} -s double -c red", NAME);
                return;
            }
            "--version" | "-V" => {
                println!("{} {}", NAME, VERSION);
                return;
            }
            "--style" | "-s" => {
                if i + 1 < args.len() {
                    style = match args[i + 1].as_str() {
                        "rounded" => &ROUNDED,
                        "double" => &DOUBLE,
                        "heavy" => &HEAVY,
                        "ascii" => &ASCII,
                        "normal" => &NORMAL,
                        _ => {
                            eprintln!("Unknown style: {}. Using normal.", args[i + 1]);
                            &NORMAL
                        }
                    };
                    skip_next = true;
                }
            }
            "--color" | "-c" => {
                if i + 1 < args.len() {
                    color = &args[i + 1];
                    skip_next = true;
                }
            }
            _ => {
                eprintln!("Unknown argument: {}", arg);
                eprintln!("Try '{} --help' for more information", NAME);
                std::process::exit(1);
            }
        }
    }
    
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    
    let text = input.trim_end_matches('\n');
    draw_box(&text, 1, style, color);
}
