pub mod words
{
    use std::char;
    use rand::prelude::*;
    use rand_chacha::ChaCha8Rng;

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

    pub fn parser(tokens: Vec<Token>) -> Result<String, &'static str> {
        let mut output = String::new();
        let mut tokens = tokens;

        let nouns_raw = include_str!("./resources/wordslist/nouns.txt").to_string();
        let mut nouns: Vec<String> = Vec::new();
        for f in nouns_raw.split("\n") {
            nouns.push(f.to_string());
        }

        let adj_raw = include_str!("./resources/wordslist/adjectives.txt").to_string();
        let mut adj_list: Vec<String> = Vec::new();
        for f in adj_raw.split("\n") {
            adj_list.push(f.to_string());
        }

        let adverbs_raw = include_str!("./resources/wordslist/adverbs.txt").to_string();
        let mut adverbs: Vec<String> = Vec::new();
        for f in adverbs_raw.split("\n") {
            adverbs.push(f.to_string());
        }

        let past_verbs_raw = include_str!("./resources/wordslist/verbs/past.txt").to_string();
        let mut past_verbs: Vec<String> = Vec::new();
        for f in past_verbs_raw.split("\n") {
            past_verbs.push(f.to_string());
        }

        let present_verbs_raw = include_str!("./resources/wordslist/verbs/present.txt").to_string();
        let mut present_verbs: Vec<String> = Vec::new();
        for f in present_verbs_raw.split("\n") {
            present_verbs.push(f.to_string());
        }

        let names_raw = include_str!("./resources/wordslist/names/surnames/names.txt").to_string();
        let mut names: Vec<String> = Vec::new();
        for f in names_raw.split("\n") {
            names.push(f.to_string());
        }

        let whitelist_raw = include_str!("./resources/wordslist/whitelist.txt").to_string();
        let mut whitelist: Vec<String> = Vec::new();
        for f in whitelist_raw.split("\n") {
            whitelist.push(f.to_string());
        }

        for token in &mut tokens {
            match token {
                Token::NUMBER(num) => {
                    *num = thread_rng().gen_range(0..*num * 2);
                    output = format!("{}{}", output, " ");
                    output = format!("{}{}", output, num.to_string());
                }
                Token::WORD(word) => {
                    if whitelist.contains(&word.to_lowercase()) {
                        output = format!("{}{}", output, " ");
                        output = format!("{}{}", output, word);
                        continue;
                    }

                    if adverbs.contains(&word.to_lowercase()) {
                        output = format!("{}{}", output, " ");
                        output = format!("{}{}", output, rand_adverb());
                        continue;
                    }

                    if adj_list.contains(&word.to_lowercase()) {
                        output = format!("{}{}", output, " ");
                        output = format!("{}{}", output, rand_adj());
                        continue;
                    }

                    if past_verbs.contains(&word.to_lowercase()) {
                        output = format!("{}{}", output, " ");
                        output = format!("{}{}", output, rand_verb_past());
                        continue;
                    }

                    if present_verbs.contains(&word.to_lowercase()) {
                        output = format!("{}{}", output, " ");
                        output = format!("{}{}", output, rand_verb_present());
                        continue;
                    }

                    if nouns.contains(&word.to_lowercase()) {
                        output = format!("{}{}", output, " ");
                        output = format!("{}{}", output, rand_noun());
                        continue;
                    }

                    if nouns.contains(&format!("{}s", &word.to_lowercase())) {
                        output = format!("{}{}", output, " ");
                        output = format!("{}{}s", output, rand_noun());
                        continue;
                    }

                    if names.contains(&word.to_lowercase()) {
                        println!("Swapping name: {}", word);
                        output = format!("{}{}", output, " ");
                        output = format!("{}{}", output, word);
                        continue;
                    }

                    output = format!("{}{}", output, " ");
                    output = format!("{}{}", output, word);

                    continue;
                }
                Token::INDENT => {
                    output = format!("{}{}", output, "  ");
                    continue;
                }
                Token::COMMA => {
                    output = format!("{}{}", output, ",");
                    continue;
                }
                Token::DIALOGUESTART => {
                    output = format!("{}{}", output, " ");
                    output = format!("{}{}", output, "\"");
                    continue;
                }
                Token::DIALOGUEEND => {
                    output = format!("{}{}", output, "\"");
                    continue;
                }
                Token::EXCLAMATION => {
                    output = format!("{}{}", output, "!");
                    continue;
                }
                Token::PERIOD => {
                    output = format!("{}{}", output, ".");
                    continue;
                }
                Token::QUESTION => {
                    output = format!("{}{}", output, "?");
                    continue;
                }
                Token::SPECIALCHAR(c) => {
                    output = format!("{}{}", output, c);
                    continue;
                }
            }
        }

        output = output.replace("\' ", "\'");
        output = output.replace("\" ", "\"");

        output = output.replace(". a", ". A");
        output = output.replace(". b", ". B");
        output = output.replace(". c", ". C");
        output = output.replace(". d", ". D");
        output = output.replace(". e", ". E");
        output = output.replace(". f", ". F");
        output = output.replace(". g", ". G");
        output = output.replace(". h", ". H");
        output = output.replace(". i", ". I");
        output = output.replace(". j", ". J");
        output = output.replace(". k", ". K");
        output = output.replace(". l", ". L");
        output = output.replace(". m", ". M");
        output = output.replace(". n", ". N");
        output = output.replace(". o", ". O");
        output = output.replace(". p", ". P");
        output = output.replace(". q", ". Q");
        output = output.replace(". r", ". R");
        output = output.replace(". s", ". S");
        output = output.replace(". t", ". T");
        output = output.replace(". u", ". U");
        output = output.replace(". v", ". V");
        output = output.replace(". w", ". W");
        output = output.replace(". x", ". X");
        output = output.replace(". y", ". Y");
        output = output.replace(". z", ". Z");

        output = output.replace("? a", "? A");
        output = output.replace("? b", "? B");
        output = output.replace("? c", "? C");
        output = output.replace("? d", "? D");
        output = output.replace("? e", "? E");
        output = output.replace("? f", "? F");
        output = output.replace("? g", "? G");
        output = output.replace("? h", "? H");
        output = output.replace("? i", "? I");
        output = output.replace("? j", "? J");
        output = output.replace("? k", "? K");
        output = output.replace("? l", "? L");
        output = output.replace("? m", "? M");
        output = output.replace("? n", "? N");
        output = output.replace("? o", "? O");
        output = output.replace("? p", "? P");
        output = output.replace("? q", "? Q");
        output = output.replace("? r", "? R");
        output = output.replace("? s", "? S");
        output = output.replace("? t", "? T");
        output = output.replace("? u", "? U");
        output = output.replace("? v", "? V");
        output = output.replace("? w", "? W");
        output = output.replace("? x", "? X");
        output = output.replace("? y", "? Y");
        output = output.replace("? z", "? Z");

        output = output.replace("! a", "! A");
        output = output.replace("! b", "! B");
        output = output.replace("! c", "! C");
        output = output.replace("! d", "! D");
        output = output.replace("! e", "! E");
        output = output.replace("! f", "! F");
        output = output.replace("! g", "! G");
        output = output.replace("! h", "! H");
        output = output.replace("! i", "! I");
        output = output.replace("! j", "! J");
        output = output.replace("! k", "! K");
        output = output.replace("! l", "! L");
        output = output.replace("! m", "! M");
        output = output.replace("! n", "! N");
        output = output.replace("! o", "! O");
        output = output.replace("! p", "! P");
        output = output.replace("! q", "! Q");
        output = output.replace("! r", "! R");
        output = output.replace("! s", "! S");
        output = output.replace("! t", "! T");
        output = output.replace("! u", "! U");
        output = output.replace("! v", "! V");
        output = output.replace("! w", "! W");
        output = output.replace("! x", "! X");
        output = output.replace("! y", "! Y");
        output = output.replace("! z", "! Z");

        Ok(output)
    }
    
    pub fn rand_noun() -> String
    {
        use rand::prelude::*;

        let file = include_str!("./resources/wordslist/nouns.txt").to_string();
        let mut lines: Vec<String> = Vec::new();
        for f in file.split("\n") {
            lines.push(f.to_string());
        }
        
        lines.to_vec()[thread_rng().gen_range(0..lines.len())].to_string()
    }

    pub fn rand_adj() -> String
    {
        use rand::prelude::*;

        let file = include_str!("./resources/wordslist/adjectives.txt").to_string();
        let mut lines: Vec<String> = Vec::new();
        for f in file.split("\n") {
            lines.push(f.to_string());
        }
        
        lines.to_vec()[thread_rng().gen_range(0..lines.len())].to_string()
    }

    pub fn rand_adverb() -> String
    {
        use rand::prelude::*;

        let file = include_str!("./resources/wordslist/adverbs.txt").to_string();
        let mut lines: Vec<String> = Vec::new();
        for f in file.split("\n") {
            lines.push(f.to_string());
        }
        
        lines.to_vec()[thread_rng().gen_range(0..lines.len())].to_string()
    }

    pub fn rand_verb_past() -> String
    {
        use rand::prelude::*;

        let file = include_str!("./resources/wordslist/verbs/past.txt").to_string();
        let mut lines: Vec<String> = Vec::new();
        for f in file.split("\n") {
            lines.push(f.to_string());
        }
        
        lines.to_vec()[thread_rng().gen_range(0..lines.len())].to_string()
    }

    pub fn rand_verb_present() -> String
    {
        use rand::prelude::*;

        let file = include_str!("./resources/wordslist/verbs/present.txt").to_string();
        let mut lines: Vec<String> = Vec::new();
        for f in file.split("\n") {
            lines.push(f.to_string());
        }
        
        lines.to_vec()[thread_rng().gen_range(0..lines.len())].to_string()
    }

    pub fn rand_name(idx: u64) -> String
    {
        use rand::prelude::*;

        let file = include_str!("./resources/wordslist/names/surnames/names.txt").to_string();
        let mut lines: Vec<String> = Vec::new();
        for f in file.split("\n") {
            lines.push(f.to_string());
        }
        
        let mut rand = ChaCha8Rng::seed_from_u64(idx);
        let mut name = lines.to_vec()[rand.gen_range(0..lines.len())].to_string();
        name.replace_range(0..1, &name.chars().nth(0).unwrap().to_string().as_str().to_uppercase());
        name
    }
}