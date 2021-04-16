use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Sankhya(pub i32);

impl fmt::Display for Sankhya {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted = self
            .0
            .to_string()
            .chars()
            .map(|c| match c {
                '-' => "-",
                '0' => "०",
                '1' => "१",
                '2' => "२",
                '3' => "३",
                '4' => "४",
                '5' => "५",
                '6' => "६",
                '7' => "७",
                '8' => "८",
                '9' => "९",
                _ => "",
            })
            .collect::<Vec<&str>>()
            .concat();

        write!(f, "{}", formatted)
    }
}

impl FromStr for Sankhya {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .graphemes(true)
            .into_iter()
            .map(|n| match n {
                "+" => "+",
                "-" => "-",
                "०" => "0",
                "१" => "1",
                "२" => "2",
                "३" => "3",
                "४" => "4",
                "५" => "5",
                "६" => "6",
                "७" => "7",
                "८" => "8",
                "९" => "9",
                _ => n,
            })
            .collect::<Vec<&str>>()
            .concat()
            .parse::<i32>()?;

        Ok(Sankhya(value))
    }
}
