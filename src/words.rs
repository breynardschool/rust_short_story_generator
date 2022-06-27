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
        let mut cur_num: i32 = 0;

        while idx < end
        {
            let cur_word_read = cur_word.clone();

            match input.as_bytes().to_vec()[idx] {
                // Comma
                44 => {
                    if !cur_word_read.to_string().is_empty() {
                        tokens.push(Token::WORD(cur_word_read));
                        cur_word = String::new();
                    }

                    if cur_num != 0 {
                        tokens.push(Token::NUMBER(cur_num));
                        cur_num = 0;
                    }

                    tokens.push(Token::COMMA);
                }
                //Period
                46 => {
                    if !cur_word_read.to_string().is_empty() {
                        tokens.push(Token::WORD(cur_word_read));
                        cur_word = String::new();
                    }

                    if cur_num != 0 {
                        tokens.push(Token::NUMBER(cur_num));
                        cur_num = 0;
                    }
                    tokens.push(Token::PERIOD);
                }
                //Exclamation
                33 => {
                    if !cur_word_read.to_string().is_empty() {
                        tokens.push(Token::WORD(cur_word_read));
                        cur_word = String::new();
                    }

                    if cur_num != 0 {
                        tokens.push(Token::NUMBER(cur_num));
                        cur_num = 0;
                    }
                    
                    tokens.push(Token::EXCLAMATION);
                }
                // Question
                63 => {
                    if !cur_word_read.to_string().is_empty() {
                        tokens.push(Token::WORD(cur_word_read));
                        cur_word = String::new();
                    }

                    if cur_num != 0 {
                        tokens.push(Token::NUMBER(cur_num));
                        cur_num = 0;
                    }

                    tokens.push(Token::QUESTION);
                }
                //Numbers
                48 => {
                    if !cur_word.to_string().is_empty() {
                        tokens.push(Token::WORD(cur_word_read));
                        cur_word = String::new();
                    }
                    cur_num *= 10;
                }
                49 => {
                    if !cur_word.to_string().is_empty() {
                        tokens.push(Token::WORD(cur_word_read));
                        cur_word = String::new();
                    }
                    cur_num *= 10;
                    cur_num += 1;
                }
                50 => {
                    if !cur_word.to_string().is_empty() {
                        tokens.push(Token::WORD(cur_word_read));
                        cur_word = String::new();
                    }
                    cur_num *= 10;
                    cur_num += 2;
                }
                51 => {
                    if !cur_word.to_string().is_empty() {
                        tokens.push(Token::WORD(cur_word_read));
                        cur_word = String::new();
                    }
                    cur_num *= 10;
                    cur_num += 3;
                }
                52 => {
                    if !cur_word.to_string().is_empty() {
                        tokens.push(Token::WORD(cur_word_read));
                        cur_word = String::new();
                    }
                    cur_num *= 10;
                    cur_num += 4;
                }
                53 => {
                    if !cur_word.to_string().is_empty() {
                        tokens.push(Token::WORD(cur_word_read));
                        cur_word = String::new();
                    }
                    cur_num *= 10;
                    cur_num += 5;
                }
                54 => {
                    if !cur_word.to_string().is_empty() {
                        tokens.push(Token::WORD(cur_word_read));
                        cur_word = String::new();
                    }
                    cur_num *= 10;
                    cur_num += 6;
                }
                55 => {
                    if !cur_word.to_string().is_empty() {
                        tokens.push(Token::WORD(cur_word_read));
                        cur_word = String::new();
                    }
                    cur_num *= 10;
                    cur_num += 7;
                }
                56 => {
                    if !cur_word.to_string().is_empty() {
                        tokens.push(Token::WORD(cur_word_read));
                        cur_word = String::new();
                    }
                    cur_num *= 10;
                    cur_num += 8;
                }
                57 => {
                    if !cur_word.to_string().is_empty() {
                        tokens.push(Token::WORD(cur_word_read));
                        cur_word = String::new();
                    }
                    cur_num *= 10;
                    cur_num += 9;
                }
                // Dialogue
                34 => {
                    if dialogue == true {
                        if !cur_word_read.to_string().is_empty() {
                            tokens.push(Token::WORD(cur_word_read));
                            cur_word = String::new();
                        }

                        if cur_num != 0 {
                            tokens.push(Token::NUMBER(cur_num));
                            cur_num = 0;
                        }
                        
                        tokens.push(Token::DIALOGUEEND);
                        dialogue = false;
                    }
                    else {
                        if !cur_word_read.to_string().is_empty() {
                            tokens.push(Token::WORD(cur_word_read));
                            cur_word = String::new();
                        }

                        if cur_num != 0 {
                            tokens.push(Token::NUMBER(cur_num));
                            cur_num = 0;
                        }

                        tokens.push(Token::DIALOGUESTART);
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
                    if cur_num != 0 {
                        tokens.push(Token::NUMBER(cur_num));
                        cur_num = 0;
                    }

                    cur_word.push(char::from_u32(input.as_bytes().to_vec()[idx] as u32).unwrap());
                }
                97..=122 => {
                    if cur_num != 0 {
                        tokens.push(Token::NUMBER(cur_num));
                        cur_num = 0;
                    }

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

                    if cur_num != 0 {
                        tokens.push(Token::NUMBER(cur_num));
                        cur_num = 0;
                    }

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

        // Final number

        if cur_num != 0 {
            tokens.push(Token::NUMBER(cur_num));
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