# QrCoder

Forker from `nkr13/qrcode-encrypt` repository!

![GitHub version](https://img.shields.io/badge/version-v0.1.2-green?style=plastic&labelColor=dark)
[![Building Project](https://github.com/breadrock1/QrCoder/actions/workflows/build-project-action.yml/badge.svg?branch=master)](https://github.com/breadrock1/QrCoder/actions/workflows/build-project-action.yml)

There is simple tool based on Rust which I forked from `nkr413/qrcode-encrypt` while researching Rust programming language. The main purpose of `QrCoder` is encode/decode text message to [QrCode](https://en.wikipedia.org/wiki/QR_code) by selected cryptinh algorithm. It is capable of intelligently recognizing several encoding formats using heuristic techniques. This rool provides several available modes: encode and decode. Encoding is the process of putting a sequence of character’s (letters, numbers, punctuation, and symbols) into a specialized format which is used for efficient transmission or storage. Decoding is the opposite process of encoding the conversion of an encoded format back into the original format. Encoding and decoding can be used in data communications, networking, and storage.

The QR code (an initialism for quick response code) is a type of matrix barcode (or two-dimensional barcode) invented in 1994 by the Japanese automotive company Denso Wave. A barcode is a machine-readable optical label that can contain information about the item to which it is attached. In practice, QR codes often contain data for a locator, identifier, or tracker that points to a website or application. The QR code is case sensitive. It uses four standardized encoding modes (numeric, alphanumeric, byte/binary, and kanji) to store data efficiently; extensions may also be used.

There are following available crypting algorithms:
 - `text`
 - `base64`
 - `hex`
 - `morse`
 - `rot13`
 - `caesar`
 - `scytale`
 - `vigenere`
 - `porta`
 
### Base64 algorithm

The Base64 algorithm is one of the algorithms for Encoding and Decoding an object into ASCII format, which is meant for the base number 64 or one of the methods used to encode the binary data. Base64 Commonly used in various applications such as e-mail via MME, XML data, or for URL encoding purposes. 
 
### Hex algorithm

The hexadecimal (also base 16 or simply hex) numeral system is a positional numeral system that represents numbers using a radix (base) of 16. Unlike the decimal system representing numbers using 10 symbols, hexadecimal uses 16 distinct symbols, most often the symbols "0"–"9" to represent values 0 to 9, and "A"–"F" (or alternatively "a"–"f") to represent values from 10 to 15.

### Morse algorithm

The Morse code is a method used in telecommunication to encode text characters as standardized sequences of two different signal durations, called dots and dashes, or dits and dahs. Morse code is named after Samuel Morse, one of the inventors of the telegraph. 

### Rot13 algorithm

The Rot13 ("rotate by 13 places", sometimes hyphenated ROT-13) is a simple letter substitution cipher that replaces a letter with the 13th letter after it in the alphabet. ROT13 is a special case of the Caesar cipher which was developed in ancient Rome.

### Caesar algorithm

In cryptography, a Caesar cipher, also known as Caesar's cipher, the shift cipher, Caesar's code or Caesar shift, is one of the simplest and most widely known encryption techniques. It is a type of substitution cipher in which each letter in the plaintext is replaced by a letter some fixed number of positions down the alphabet.

### Scytale algorithm 

In cryptography, a scytale (/ˈskɪtəliː/; also transliterated skytale, Ancient Greek: σκυτάλη skutálē "baton, cylinder", also σκύταλον skútalon) is a tool used to perform a transposition cipher, consisting of a cylinder with a strip of parchment wound around it on which is written a message. The ancient Greeks, and the Spartans in particular, are said to have used this cipher to communicate during military campaigns.

### Vigenere algorithm

The Vigenere cipher is a method of encrypting alphabetic text by using a series of interwoven Caesar ciphers, based on the letters of a keyword. It employs a form of polyalphabetic substitution.
 
### Porta algorithm

The Porta Cipher is a polyalphabetic substitution cipher invented by Giovanni Battista della Porta. Where the Vigenere cipher is a polyalphabetic cipher with 26 alphabets, the Porta is basically the same except it only uses 13 alphabets. The 13 cipher alphabets it uses are reciprocal, so enciphering is the same as deciphering.

The algorithm used here is the same as that used by the American Cryptogram Association. Another source is Helen Fouche Gaines book "Cryptanalysis".



## Building tool

There is command to build Rust project:

```shell
    cargo build 
```
## Launching tool

```shell
Usage: qrcoder [OPTIONS] --algorithm <CIPHER> <MODE>

Arguments:
  <MODE>  The application mode: generate or extract

Options:
  -a, --algorithm <CIPHER>  The en/decoding algorithm.
  -k, --key <VALUE>         The key to apply cipher.
  -f, --file <PATH>         Path to file with QrCode
  -o, --output <PATH>       The output file path.
  -m, --msg <VALUE>         The message to encode.
  -h, --help                Print help information
  -V, --version             Print version information
```

So you can launch tool with like those command examples to generate QrCode:

```shell
./qrcode generate -a rot13 -f /tmp/message.txt -o /tmp/qrcode.png
```

```shell
./qrcode generate -a rot13 -m "Some message!" -o /tmp/qrcode.png
```

```shell
./qrcode generate -a porta -k "KEY" -f /tmp/message.txt -o /tmp/qrcode.png
```

And those commands to extract message from QrCode:

```shell
./qrcode extract -a rot13 -f /tmp/qrcode.png -o /tmp/message.txt
```

```shell
./qrcode extract -a porta -k "KEY" -f /tmp/qrcode.png -o /tmp/message.txt
```

#### Attention

There are several algorithms which require `-k, --key <VALUE>` key option such as:

 - `caesar`     - key type: `Number`;
 - `scytale`    - key type: `Number`;
 - `vigenere`   - key type: `String`;
 - `porta`      - key type: `String`.
 
