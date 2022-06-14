pub mod words
{
    pub fn rand_noun() -> String
    {
        use std::{
            fs::File,
            io::{prelude::*, BufReader}
        };

        use rand::prelude::*;

        let file = File::open("./src/resources/words/nouns.txt").expect("no such file");
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

        let file = File::open("./src/resources/words/adjectives.txt").expect("no such file");
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

        let file = File::open("./src/resources/words/names/surnames/english.txt").expect("no such file");
        let buf = BufReader::new(file);
        let lines: Vec<String> = buf.lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();
        
        let mut name = lines.to_vec()[thread_rng().gen_range(0..671)].to_string();
        name.replace_range(0..1, &name.chars().nth(0).unwrap().to_string().as_str().to_uppercase());
        name
    }
}