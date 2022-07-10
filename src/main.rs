use rust_short_story_generator::words::words::*;
use rand::prelude::*;

const SENTENCE_COUNT: i32 = 4;

fn main() 
{
    let mut sentences: Vec<Token> = Vec::new();

    let crabbattles = include_str!("./resources/inputfiles/crabbattles.txt");
    let sickking = include_str!("./resources/inputfiles/sickking.txt");
    let star = include_str!("./resources/inputfiles/star.txt");

    let mut data: Vec<String> = Vec::new();

    for s in crabbattles.split(". ") {
        data.push(s.to_string());
    }

    for s in sickking.split(". ") {
        data.push(s.to_string());
    }

    for s in star.split(". ") {
        data.push(s.to_string());
    }

    // Lexing
    let mut i = 0;
    while i < SENTENCE_COUNT {
        let cur_sentence = &data[thread_rng().gen_range(0..data.len())];

        let tokens = lexer(cur_sentence.to_string());
        for t in tokens {
            sentences.push(t);
        }

        i += 1;
    }

    let story = parser(sentences).unwrap();
    println!("{}", story);
}