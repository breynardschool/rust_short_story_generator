pub mod words
{
    use std::char;

    pub enum WordType
    {
        ADJECTIVE,
        VERB,
        NOUN,
        TRANSITION,
        ADVERB,
        NAME
    }

    #[derive(Debug)]
    pub enum Token
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

    pub fn lexer(input: String) -> Vec<Token>
    {
        // Initialization
        let mut tokens: Vec<Token> = Vec::new();
        
        let mut idx: usize = 0;
        let end: usize = input.len();

        // Token seperation variables
        let mut dialogue: bool = false;
        let mut cur_word: String = String::new();

        while idx < end
        {
            let cur_word_read = cur_word.clone();

            match input.as_bytes().to_vec()[idx] {
                // Comma
                44 => {
                    tokens.push(Token::WORD(cur_word_read));
                    cur_word = String::new();
                    tokens.push(Token::COMMA);
                }
                //Period
                46 => {
                    tokens.push(Token::WORD(cur_word_read));
                    cur_word = String::new();
                    tokens.push(Token::PERIOD);
                }
                //Exclamation
                33 => {
                    tokens.push(Token::WORD(cur_word_read));
                    cur_word = String::new();
                    tokens.push(Token::EXCLAMATION);
                }
                // Question
                63 => {
                    tokens.push(Token::WORD(cur_word_read));
                    cur_word = String::new();
                    tokens.push(Token::QUESTION);
                }
                //Numbers
                48 => {
                    tokens.push(Token::WORD(cur_word_read));
                    cur_word = String::new();
                    tokens.push(Token::NUMBER(0));
                }
                49 => {
                    tokens.push(Token::WORD(cur_word_read));
                    cur_word = String::new();
                    tokens.push(Token::NUMBER(1));
                }
                50 => {
                    tokens.push(Token::WORD(cur_word_read));
                    cur_word = String::new();
                    tokens.push(Token::NUMBER(2));
                }
                51 => {
                    tokens.push(Token::WORD(cur_word_read));
                    cur_word = String::new();
                    tokens.push(Token::NUMBER(3));
                }
                52 => {
                    tokens.push(Token::WORD(cur_word_read));
                    cur_word = String::new();
                    tokens.push(Token::NUMBER(4));
                }
                53 => {
                    tokens.push(Token::WORD(cur_word_read));
                    cur_word = String::new();
                    tokens.push(Token::NUMBER(5));
                }
                54 => {
                    tokens.push(Token::WORD(cur_word_read));
                    cur_word = String::new();
                    tokens.push(Token::NUMBER(6));
                }
                55 => {
                    tokens.push(Token::WORD(cur_word_read));
                    cur_word = String::new();
                    tokens.push(Token::NUMBER(7));
                }
                56 => {
                    tokens.push(Token::WORD(cur_word_read));
                    cur_word = String::new();
                    tokens.push(Token::NUMBER(8));
                }
                57 => {
                    tokens.push(Token::WORD(cur_word_read));
                    cur_word = String::new();
                    tokens.push(Token::NUMBER(9));
                }
                // Dialogue
                34 => {
                    if dialogue == true {
                        tokens.push(Token::DIALOGUEEND);
                        tokens.push(Token::WORD(cur_word_read));
                        cur_word = String::new();
                        dialogue = false;
                    }
                    else {
                        tokens.push(Token::DIALOGUESTART);
                        tokens.push(Token::WORD(cur_word_read));
                        cur_word = String::new();
                        dialogue = true;
                    }

                }
                // Indent
                9 => {
                    tokens.push(Token::INDENT);
                    tokens.push(Token::WORD(cur_word_read));
                    cur_word = String::new();
                    dialogue = false;
                }
                // Words
                65..=89 => {
                    cur_word.push(char::from_u32(input.as_bytes().to_vec()[idx] as u32).unwrap());
                }
                97..=122 => {
                    cur_word.push(char::from_u32(input.as_bytes().to_vec()[idx] as u32).unwrap());
                }
                //Space
                32 => {
                    if !cur_word_read.to_string().is_empty() {
                        tokens.push(Token::WORD(cur_word_read));
                        cur_word = String::new();
                    }
                }
                _ => {
                    if !cur_word_read.to_string().is_empty() {
                        tokens.push(Token::WORD(cur_word_read));
                        cur_word = String::new();
                    }

                    let c = input.as_bytes().to_vec()[idx] as u32;
                    let literal = char::from_u32(input.as_bytes().to_vec()[idx] as u32).unwrap();
                    
                    if c >= 35 && c <= 47
                    {
                        tokens.push(Token::SPECIALCHAR(literal));
                    }

                    if c >= 58 && c <= 64
                    {
                        tokens.push(Token::SPECIALCHAR(literal));
                    }

                    if c >= 91 && c <= 96
                    {
                        tokens.push(Token::SPECIALCHAR(literal));
                    }

                    if c >= 123 && c <= 126
                    {
                        tokens.push(Token::SPECIALCHAR(literal));
                    }

                    if c >= 128
                    {
                        tokens.push(Token::SPECIALCHAR(literal));
                    }
                }
            }

            idx += 1;
        }

        // Final word
        if !cur_word.to_string().is_empty() {
            tokens.push(Token::WORD(cur_word));
        }

        tokens
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