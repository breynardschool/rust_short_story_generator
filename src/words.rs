pub mod words
{
    use std::ops::Index;

    pub enum WordType
    {
        ADJECTIVE,
        VERB,
        NOUN,
        TRANSITION,
        ADVERB,
        NAME
    }

    enum Token
    {
        WORD(String),
        COMMA,
        PERIOD,
        EXCLAMATION,
        QUESTION,
        SPECIALCHAR(char),
        NUMBER(i32),
        DIALOGUESTART,
        DIALOGUEEND,
        INDENT
    }

    pub fn get_structure(input: String) -> Vec<WordType>
    {
        //Initialization
        let mut output: Vec<WordType> = Vec::new();
        let adj_list = std::fs::read_to_string(".\\src\\resources\\adjectives.txt").unwrap();
        let noun_list = std::fs::read_to_string(".\\src\\resources\\nouns.txt").unwrap();

        //Lexer
        let mut tokens: Vec<Token> = Vec::new();
        
        let mut idx: usize = 0;
        let end: usize = input.len();

        // Token seperation variables
        let mut dialogue: bool = false;
        let mut cur_word: String = String::new();

        while idx < end
        {
            let cur_word_read = cur_word.clone();

            match input.as_bytes()[idx] {

                _ => {
                    if !cur_word_read.to_string().is_empty() {
                        tokens.push(Token::WORD(cur_word_read));
                        cur_word = String::new();
                    }
                }
            }

            idx += 1;
        }

        output
    }
    
    pub fn rand_noun() -> String
    {
        use std::{
            fs::File,
            io::{prelude::*, BufReader}
        };

        use rand::prelude::*;

        let file = File::open("./src/resources/wordslist/nouns.txt").expect("no such file");
        let buf = BufReader::new(file);
        let lines: Vec<String> = buf.lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();
        
        lines.to_vec()[thread_rng().gen_range(0..1525)].to_string()
    }

    pub fn rand_adj() -> String
    {
        use std::{
            fs::File,
            io::{prelude::*, BufReader}
        };

        use rand::prelude::*;

        let file = File::open("./src/resources/wordslist/adjectives.txt").expect("no such file");
        let buf = BufReader::new(file);
        let lines: Vec<String> = buf.lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();
        
        lines.to_vec()[thread_rng().gen_range(0..1347)].to_string()
    }

    pub fn rand_name() -> String
    {
        use std::{
            fs::File,
            io::{prelude::*, BufReader}
        };

        use rand::prelude::*;

        let file = File::open("./src/resources/wordslist/names/surnames/english.txt").expect("no such file");
        let buf = BufReader::new(file);
        let lines: Vec<String> = buf.lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();
        
        let mut name = lines.to_vec()[thread_rng().gen_range(0..671)].to_string();
        name.replace_range(0..1, &name.chars().nth(0).unwrap().to_string().as_str().to_uppercase());
        name
    }
}