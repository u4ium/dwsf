use std::io::stdin;

use clap::Parser;

use itertools::Itertools;
use wordle55::find_words_with_disjoint_character_sets;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Size of clique to search for
    #[clap(short, long, value_parser)]
    num_words_per_clique: usize,

    /// Size of word to search for
    #[clap(short, long, value_parser)]
    length_of_word: usize,

    /// Allow for word lengths that exceed length_of_word in length
    #[clap(short, long, action)]
    allow_repeat_letters: bool,
}

pub fn main() -> Result<(), String> {
    let Cli {
        num_words_per_clique,
        length_of_word,
        allow_repeat_letters,
    } = Cli::parse();
    let finder = get_finder(num_words_per_clique, length_of_word)?;

    let i = get_input(if !allow_repeat_letters {
        Some(length_of_word)
    } else {
        None
    })?;

    let input = i.iter().map(|w| &w[..]).collect_vec();

    for set in finder(input) {
        println!("{}", set.join(","));
    }

    Ok(())
}

fn get_input<'a>(letter_restriction: Option<usize>) -> Result<Vec<String>, String> {
    let mut ret = vec![];

    for (i, line) in stdin().lines().enumerate() {
        let word = line.map_err(|e| e.to_string())?;

        if !word.as_bytes().into_iter().all(u8::is_ascii_alphabetic) {
            return Err(format! {"word {i} has non-letter characters: {word}"});
        }

        if let Some(expected_length_of_word) = letter_restriction {
            let length_of_word = word.len();
            if expected_length_of_word != length_of_word {
                return Err(
                    format! {"word {i} has length not equal to {expected_length_of_word}: {length_of_word}"},
                );
            }
        }

        ret.push(word);
    }

    Ok(ret)
}

// TODO: macro
fn get_finder(
    num_words_per_clique: usize,
    length_of_word: usize,
) -> Result<&'static dyn for<'a> Fn(Vec<&'a str>) -> Vec<Vec<&'a str>>, String> {
    Ok(match (num_words_per_clique, length_of_word) {
        (2, 13) => &|words| {
            find_words_with_disjoint_character_sets::<2, 13>(words)
                .iter()
                .map(|set| set.to_vec())
                .collect()
        },
        (2, 12) => &|words| {
            find_words_with_disjoint_character_sets::<2, 12>(words)
                .iter()
                .map(|set| set.to_vec())
                .collect()
        },
        (2, 11) => &|words| {
            find_words_with_disjoint_character_sets::<2, 11>(words)
                .iter()
                .map(|set| set.to_vec())
                .collect()
        },
        (2, 10) => &|words| {
            find_words_with_disjoint_character_sets::<2, 10>(words)
                .iter()
                .map(|set| set.to_vec())
                .collect()
        },
        (2, 9) => &|words| {
            find_words_with_disjoint_character_sets::<2, 9>(words)
                .iter()
                .map(|set| set.to_vec())
                .collect()
        },
        (3, 8) => &|words| {
            find_words_with_disjoint_character_sets::<3, 8>(words)
                .iter()
                .map(|set| set.to_vec())
                .collect()
        },
        (4, 6) => &|words| {
            find_words_with_disjoint_character_sets::<4, 6>(words)
                .iter()
                .map(|set| set.to_vec())
                .collect()
        },
        (5, 5) => &|words| {
            find_words_with_disjoint_character_sets::<5, 5>(words)
                .iter()
                .map(|set| set.to_vec())
                .collect()
        },
        (6, 4) => &|words| {
            find_words_with_disjoint_character_sets::<6, 4>(words)
                .iter()
                .map(|set| set.to_vec())
                .collect()
        },
        (7, 3) => &|words| {
            find_words_with_disjoint_character_sets::<7, 3>(words)
                .iter()
                .map(|set| set.to_vec())
                .collect()
        },
        _ => {
            return Err(format!(
                "Unsupported clique and word size combination: {num_words_per_clique}, {length_of_word}"
            ))
        }
    })
}
