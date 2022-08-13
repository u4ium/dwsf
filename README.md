# DWSF - The Disjoint Word Set Finder

Finds sets (of length `n`) of words that have disjoint character sets (each of length `l`).

A "character set" is "disjoint" from another if it shares no characters in common. For example, "ab" is disjoint from "cd", but "ab" is _not_ disjoint from "bc".

A "word"/"character set" has length `l` if either of the following are true:

- The "word" is exactly `l` characters long, with no repeated letters
- The "word" has exactly `l` _distinct_ letters and the `-a`/`--allow-repeat-letters` parameter is set (true).

## Example 

```bash
echo abcde fghij abcfj qwzxy aa | dwsf -n2 -l5
abcde,fghij
abcde,qwzxy
abcfj,qwzxy
fghij,qwzxy
```

Note that these are mandatory:

- The `-n` parameter is the size of the set to search for
- The `-l` parameter is the "length of the word"/"number of distinct characters" to search for

### From a file 

Although `cat res/all_words.txt | dwsf ...` works, and is a good solution, `dwsf` also supports:

```bash
dswf -n5 -l5 res/all_words.txt
```

which allows it to read directly from a file, for convenience (like any good command-line program, it accepts input from `stdin`, or a specified `WORD_FILE_PATH`).

## Usage

```bash
dwfs --help
```

```
disjoint_word_set_finder 0.1.0
Find sets of n words, that do not share characters, from a given list of words with length l

USAGE:
    dwsf.exe [OPTIONS] --num-words-per-set <NUM_WORDS_PER_SET> --length-of-word <LENGTH_OF_WORD> [WORD_FILE_PATH]

ARGS:
    <WORD_FILE_PATH>    An optional path to the word list file (otherwise read from stdin)

OPTIONS:
    -a, --allow-repeat-letters
            Allow for word lengths that exceed `length_of_word`

    -h, --help
            Print help information

    -l, --length-of-word <LENGTH_OF_WORD>
            Size of word to search for

    -n, --num-words-per-set <NUM_WORDS_PER_SET>
            Size of set to search for

    -V, --version
            Print version information
```

## Compilation

- Install the [rust toolchain](https://www.rust-lang.org/tools/install)
- `cargo build --release` (debug is slow for big word sets)

### Test

```bash
cargo test
```