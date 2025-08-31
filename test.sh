# Mixed content
echo -e "Wide emojis (2 cols): 🚀🌟🔥\nNarrow symbols (1 col): ✨❤️\nMixed: Orange text 🚀 and ✨" | boxy

# Your test case
echo -e "Red text\nGreen text\nBlue text with emoji 🚀\nNormal text\nMixed red and green 🌟" | boxy

# Fun example
echo -e "📦 boxy v0.1.0\n🎯 Perfect alignment\n🦀 Written in Rust\n⚡ Lightning fast" | boxy

# Emoji overload
echo -e "🍕🍔🌮🌯🥙\n🍎🍊🍋🍌🍉\n🚗🚕🚙🚌🚎" | boxy

# Colored boxes
echo "Hello World" | boxy --color red
echo "Hello World" | boxy -c orange

# Combine style and color
echo "Hello World" | boxy --style rounded --color blue
echo "Hello World" | boxy -s double -c purple

# Your special colors
echo "Deep color" | boxy -c deep
echo "Deep green" | boxy -c deep_green
echo "Grey shades" | boxy -c grey2

# With emojis and colors
echo -e "🎉 Party\n🚀 Time" | boxy -s rounded -c cyan
