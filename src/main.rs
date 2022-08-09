use std::io::Read;

use wordle55::find_words_with_disjoint_character_sets;

pub fn main() -> Result<(), String> {
    let mut buffer = String::new();
    std::io::stdin()
        .read_to_string(&mut buffer)
        .map_err(|e| e.to_string())?;

    let input = parse_input(&buffer)?;

    // TODO: take command-line args for <5, 5>
    let result = find_words_with_disjoint_character_sets::<5, 5>(input);

    for set in result {
        println!("{}", set.join(","));
    }

    Ok(())
}

fn parse_input<'a>(input: &'a String) -> Result<Vec<&'a str>, String> {
    let ret: Vec<&str> = input.split_whitespace().collect();
    for (i, word) in ret.iter().enumerate() {
        if !word.as_bytes().into_iter().all(u8::is_ascii_alphabetic) {
            return Err(format! {"word {i} has non-letter characters: {word}"});
        }
    }
    Ok(ret)
}
