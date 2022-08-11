use std::{
    fs,
    io::{stdin, BufRead, BufReader},
};

use clap::Parser;

use itertools::Itertools;

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

    let letter_restriction = if !allow_repeat_letters {
        Some(length_of_word)
    } else {
        None
    };
    let words = get_input(word_file_path, letter_restriction)?;
    let input = words.iter().map(String::as_str).collect_vec();

    for set in finder(input) {
        println!("{}", set.join(","));
    }

    Ok(())
}

fn get_input<'a>(
    word_file_path: Option<String>,
    letter_restriction: Option<usize>,
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

        if !word.as_bytes().into_iter().all(u8::is_ascii_alphabetic) {
            return Err(format! {"word {i} has non-letter characters: {word}"});
        }

        if let Some(expected_length_of_word) = letter_restriction {
            let length_of_word = word.len();
            if expected_length_of_word != length_of_word {
                continue;
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
        (2, 13) => finder_wrapper::<2, 13>(),
        (2, 12) => finder_wrapper::<2, 12>(),
        (2, 11) => finder_wrapper::<2, 11>(),
        (2, 10) => finder_wrapper::<2, 10>(),
        (2, 9) => finder_wrapper::<2, 9>(),
        (3, 8) => finder_wrapper::<3, 8>(),
        (4, 6) => finder_wrapper::<4, 6>(),
        (5, 5) => finder_wrapper::<5, 5>(),
        (6, 4) => finder_wrapper::<6, 4>(),
        _ => {
            return Err(format!(
            "Unsupported clique and word size combination: {num_words_per_set}, {length_of_word}"
        ))
        }
    })
}
