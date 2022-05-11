#!/bin/sh

encode_png() {
  pngchat encode ./test.png ruSt "This is a hidden message"
}

decode_png() {
  pngchat decode ./test.png ruSt
}

remove_png() {
  pngchat remove ./test.png ruSt
}

print_png() {
  pngchat print ./test.png
}

echo "Encoding message..."
encode_png

decode_png

echo "Removing message..."
remove_png

print_png