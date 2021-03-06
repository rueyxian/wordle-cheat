use crate::pattern;

use regex::Regex;

// ===============================================

#[derive(Debug)]
pub enum Error {
    TryFromStr(String),
    TryFromStrStr(String, String),
}

// ===============================================

#[derive(Debug, Clone)]
pub struct PatternLine {
    pub units: Vec<pattern::PatternUnit>,
}

impl TryFrom<&str> for PatternLine {
    type Error = self::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let s = value.to_lowercase();
        let regex =
            Regex::new(r"^(?P<units>(?:#[a-z]|\?[a-z]|![a-z])(?:\s(?:#[a-z]|\?[a-z]|![a-z]))*)$")
                .unwrap();
        let caps = regex
            .captures(s.as_str())
            .ok_or(Error::TryFromStr(value.to_owned()))?;
        let cap = caps.name("units").map_or("", |m| m.as_str()).trim_start();
        let mut units = Vec::<pattern::PatternUnit>::with_capacity(cap.len());
        for letters in cap.split_whitespace() {
            let mut letters_iter = letters.chars();
            match letters_iter.next() {
                Some('!') => {
                    units.push(pattern::PatternUnit::Correct(letters_iter.next().unwrap()))
                }
                Some('?') => units.push(pattern::PatternUnit::Wrong(letters_iter.next().unwrap())),
                Some('#') => units.push(pattern::PatternUnit::NotAny(letters_iter.next().unwrap())),
                _ => unreachable!(),
            };
        }
        Ok(PatternLine { units })
    }
}

impl TryFrom<(&str, &str)> for PatternLine {
    type Error = self::Error;
    fn try_from((word, pattern): (&str, &str)) -> Result<Self, Self::Error> {
        if word.len() != pattern.len() {
            return Err(Error::TryFromStrStr(word.to_owned(), pattern.to_owned()));
        }
        let mut units = Vec::<pattern::PatternUnit>::with_capacity(word.len());
        for (c, u) in word.to_lowercase().chars().zip(pattern.chars()) {
            units.push(match u {
                '!' => pattern::PatternUnit::Correct(c),
                '?' => pattern::PatternUnit::Wrong(c),
                '#' => pattern::PatternUnit::NotAny(c),
                _ => return Err(Error::TryFromStrStr(word.to_owned(), pattern.to_owned())),
            });
        }
        Ok(PatternLine { units })
    }
}

impl std::fmt::Display for PatternLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .units
            .iter()
            .map(|unit| unit.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "{}", s)
    }
}
