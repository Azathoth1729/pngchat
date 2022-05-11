# pngchat

Hide messages in the PNG file.

The intent of this little project is to learn how to encode PNG file and add some messages inside it

Idea come from [PNGme: An Intermediate Rust Project](https://picklenerd.github.io/pngme_book/introduction.html)

## Goal

Making a command line program that lets you hide secret messages in PNG files.

The main tasks of `pngchat` are:

- Encode a message into a PNG file
- Decode a message stored in a PNG file
- Remove a message from a PNG file
- Print a list of PNG chunks that can be searched for messages

## Uasge

```bash
//Encodes a message into a PNG file and saves the result
pngchat encode ./test.png ruSt "This is a hidden message"

// Searches for a message hidden in a PNG file and prints the message if one is found
pngchat decode ./test.png ruSt

// Removes a chunk from a PNG file and saves the result
pngchat remove ./test.png ruSt

// Prints all of the chunks in a PNG file
pngchat print ./test.png
```

## Links

See the [PNG file structure spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html) for more details about how PNG file structured

## License

This project is licensed under the [MIT license](LICENSE).
