use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // Create a hash map mapping keywords to emoji characters
    let mut emoji = std::collections::HashMap::new();
    emoji.insert("smile", "😃");
    emoji.insert("bear", "🐻");
    emoji.insert("hamburger", "🍔");
    emoji.insert("lightbulb", "💡");
    emoji.insert("idea", "💡");
    emoji.insert("comment", "💬");
    emoji.insert("chat", "💬");
    emoji.insert("pomo", "🍅");
    emoji.insert("stop", "🛑");
    emoji.insert("warning", "⚠️");
    emoji.insert("rant", "🤬");
    emoji.insert("tv", "📺");
    emoji.insert("update", "📰");
    emoji.insert("tux", "🐧");
    emoji.insert("facepalm", "🤦");
    emoji.insert("puke", "🤢");
    emoji.insert("skull", "💀");
    emoji.insert("wizard", "🧙");

    // Get the input file name from the command line arguments
    let file_name = env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: emoji_replacer INPUT_FILE");
        std::process::exit(1);
    });

    // Read the input file
    let input = fs::read_to_string(file_name.clone())?;

    // Replace all occurrences of the keywords with the corresponding emoji characters
    let mut output = input;
    for (keyword, emoji_char) in &emoji {
        output = output.replace(&format!(":{}:", keyword), emoji_char);
    }

    // Write the modified text to the output file
    fs::write(file_name, output)?;

    Ok(())
}

