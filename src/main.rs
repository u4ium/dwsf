use std::{
    fs,
    io::{stdin, BufRead, BufReader},
};

use clap::Parser;

use disjoint_word_set_finder::find_words_with_disjoint_character_sets;

#[derive(Parser)]
#[clap(version, about)]
struct Cli {
    /// Size of set to search for
    #[clap(short, long, value_parser)]
    num_words_per_set: usize,

    /// Size of word to search for
    #[clap(short, long, value_parser)]
    length_of_word: usize,

    /// Allow for word lengths that exceed `length_of_word`
    #[clap(short, long, action)]
    allow_repeat_letters: bool,

    /// An optional path to the word list file
    /// (otherwise read from stdin)
    #[clap(value_parser)]
    word_file_path: Option<String>,
}

pub fn main() -> Result<(), String> {
    let Cli {
        num_words_per_set,
        length_of_word,
        allow_repeat_letters,
        word_file_path,
    } = Cli::parse();
    let finder = get_finder(num_words_per_set, length_of_word)?;

    let length_restriction = if !allow_repeat_letters {
        Some(length_of_word)
    } else {
        None
    };
    let words = get_input(word_file_path, length_restriction)?;
    let input = words
        .iter()
        .filter(|s| s.len() >= length_of_word)
        .map(String::as_str)
        .collect::<Vec<_>>();

    for set in finder(input) {
        println!("{}", set.join(","));
    }

    Ok(())
}

fn get_input<'a>(
    word_file_path: Option<String>,
    length_restriction: Option<usize>,
) -> Result<Vec<String>, String> {
    let reader: Box<dyn BufRead> = match word_file_path {
        None => Box::new(BufReader::new(stdin())),
        Some(filename) => Box::new(BufReader::new(
            fs::File::open(filename).map_err(|e| e.to_string())?,
        )),
    };

    let mut ret = vec![];
    for (i, line) in reader.lines().enumerate() {
        let word = line.map_err(|e| e.to_string())?;

        // If the word is NOT alphabetic ASCII, reject with ERROR
        {
            let is_alphabetic = word.as_bytes().iter().all(u8::is_ascii_alphabetic);
            if !is_alphabetic {
                let index = i + 1;
                return Err(format! {"Word {index} has non-letter characters: {word}"});
            }
        }

        // If length is restricted AND the word is NOT the correct length, SKIP it
        {
            if let Some(expected_length_of_word) = length_restriction {
                if expected_length_of_word != word.len() {
                    continue;
                }
            }
        }

        ret.push(word);
    }

    Ok(ret)
}

fn get_finder(
    num_words_per_set: usize,
    length_of_word: usize,
) -> Result<&'static dyn for<'a> Fn(Vec<&'a str>) -> Vec<Vec<&'a str>>, String> {
    // TODO: avoid reallocating vectors
    fn finder_wrapper<const N: usize, const L: u32>(
    ) -> &'static dyn for<'a> Fn(Vec<&'a str>) -> Vec<Vec<&'a str>> {
        &|words| {
            find_words_with_disjoint_character_sets::<N, L>(words)
                .iter()
                .map(|set| set.to_vec())
                .collect()
        }
    }

    // TODO: macro
    Ok(match (num_words_per_set, length_of_word) {
        (2, 2) => finder_wrapper::<2, 2>(),
        (2, 3) => finder_wrapper::<2, 3>(),
        (2, 4) => finder_wrapper::<2, 4>(),
        (2, 5) => finder_wrapper::<2, 5>(),
        (2, 6) => finder_wrapper::<2, 6>(),
        (2, 7) => finder_wrapper::<2, 7>(),
        (2, 8) => finder_wrapper::<2, 8>(),
        (2, 9) => finder_wrapper::<2, 9>(),
        (2, 10) => finder_wrapper::<2, 10>(),
        (2, 11) => finder_wrapper::<2, 11>(),
        (2, 12) => finder_wrapper::<2, 12>(),
        (2, 13) => finder_wrapper::<2, 13>(),
        (3, 2) => finder_wrapper::<3, 2>(),
        (3, 3) => finder_wrapper::<3, 3>(),
        (3, 4) => finder_wrapper::<3, 4>(),
        (3, 5) => finder_wrapper::<3, 5>(),
        (3, 6) => finder_wrapper::<3, 6>(),
        (3, 7) => finder_wrapper::<3, 7>(),
        (3, 8) => finder_wrapper::<3, 8>(),
        (4, 2) => finder_wrapper::<4, 2>(),
        (4, 3) => finder_wrapper::<4, 3>(),
        (4, 4) => finder_wrapper::<4, 4>(),
        (4, 5) => finder_wrapper::<4, 5>(),
        (4, 6) => finder_wrapper::<4, 6>(),
        (5, 2) => finder_wrapper::<5, 2>(),
        (5, 3) => finder_wrapper::<5, 3>(),
        (5, 4) => finder_wrapper::<5, 4>(),
        (5, 5) => finder_wrapper::<5, 5>(),
        (6, 2) => finder_wrapper::<6, 2>(),
        (6, 3) => finder_wrapper::<6, 3>(),
        (6, 4) => finder_wrapper::<6, 4>(),
        _ => {
            return Err(format!(
            "Unsupported clique and word size combination: {num_words_per_set}, {length_of_word}"
        ))
        }
    })
}
