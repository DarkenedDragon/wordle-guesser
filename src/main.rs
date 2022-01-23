use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;

#[derive(PartialEq, Eq, Debug, Default)]
struct Guess {
    in_word: bool,
    wrong_pos: Option<Vec<usize>>,
    correct_pos: Option<usize>
}


fn main() {
    if let Ok(file) = File::open("word_list.txt") {
        // The data structure
        let mut words: HashSet<String> = HashSet::new();
        // Read in the word list

        println!("Loading");
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(word) = line {
                words.insert(word);
            }
        }
        println!("Done");

        // The main bits
        let mut letters_guessed = HashMap::<char, Guess>::new();
        for attempt in 0..5 {
            match attempt {
                0 => println!("First guess: "),
                1 => println!("Second guess: "),
                2 => println!("Third guess: "),
                3 => println!("Fourth guess: "),
                4 => println!("Fifth guess: "),
                _ => ()
            }

            let mut guess= String::new();
            if let Ok(_) = io::stdin().read_line(&mut guess) {
                // If there was something, the string is in guess
                let guess = guess.trim();
                println!("You guessed: {}", guess);
                // Split the word and add it to letters within
                println!("Indicate if the letter is present (l), present and correct (;), or absent (\')");
                for letter in guess.char_indices() {
                    let l_contains = letters_guessed.contains_key(&letter.1);
                    let l_incorrect = if let Some(val) = letters_guessed.get(&letter.1) {
                        val.correct_pos == None && val.in_word
                    } else {
                        false
                    };
                    if !letter.1.is_whitespace() && (!l_contains || l_incorrect) {
                        println!("{}? ", letter.1);

                        let mut present= String::new();
                        if let Ok(_) = io::stdin().read_line(&mut present) {
                            let res = present.trim();

                            if !l_incorrect {
                                match res.to_lowercase().as_str() {
                                    "l" => letters_guessed.insert(letter.1, Guess { in_word: true, wrong_pos: Some(vec!(letter.0)), correct_pos: None}),
                                    ";" => letters_guessed.insert(letter.1,Guess {  in_word: true, wrong_pos: None, correct_pos: Some(letter.0)}),
                                    _ => letters_guessed.insert(letter.1,Guess {  in_word: false, wrong_pos: None, correct_pos: None}),
                                };
                            } else {
                                match res.to_lowercase().as_str() {
                                    "l" => {
                                        if let Some(guess) = letters_guessed.get_mut(&letter.1) {
                                            if let Some(mut pos) = guess.wrong_pos.clone() {
                                                pos.push(letter.0);
                                                guess.wrong_pos = Some(pos); // this is the stupidest thing
                                            }
                                        }
                                    }
                                    ";" => {
                                        if let Some(guess) = letters_guessed.get_mut(&letter.1) {
                                            guess.correct_pos = Some(letter.0);
                                        }
                                    }
                                    _ => (),
                                };
                            }

                        }
                    }

                }
            }

            println!("letters: {:?}", letters_guessed);

            // Remove all those words that don't have the letters we need
            words.retain(|x| {
                let mut contained = true;
                for l in letters_guessed.keys() {
                    if let Some(guess) = letters_guessed.get(l) {
                        let x_contains = x.contains(l.to_owned());
                        if (!guess.in_word && x_contains) || (guess.in_word && !x_contains) {
                            contained = false;
                        }
                        // Check that letters aren't in the wrong places
                        if let Some(wrong_positions) = &guess.wrong_pos {
                            for pos in wrong_positions {
                                if let Some(letter) = x.chars().nth(pos.to_owned()) {
                                    if letter == *l {
                                        contained = false;
                                    }
                                }
                            }
                        }
                        // Check that the letter is in the correct position, if applicable
                        if let Some(pos) = &guess.correct_pos {
                            if let Some(letter) = x.chars().nth(pos.to_owned()) {
                                if letter != *l {
                                    contained = false;
                                }
                            }
                        }
                    }
                }
                return contained
            });

            // Show 3 suggestions
            if words.len() > 0 {
                println!("{} possible words", words.len());
                println!("Suggestions");
                let mut i = 0;
                for w in &words {
                    if i > 2 {
                        break;
                    }
                    println!("{}", w);
                    i += 1;
                }
            }
        }



    } else {
        println!("Could not read world list. Make sure it is named \"word_list.txt\" and is in the correct directory");
    }
}
