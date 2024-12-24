use std::{error::Error, fmt::Display, fs, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let machines = fs::read_to_string("demo.txt")?
        .split("\n\n")
        .map(|block| ClawMachine::from_str(block))
        .collect::<Result<Vec<_>, _>>()?;

    machines.iter().for_each(|m| println!("{m:?}"));

    Ok(())
}

#[derive(Debug)]
struct ClawMachine {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

impl FromStr for ClawMachine {
    type Err = ClawError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let machine_lines: Vec<_> = s.lines().collect();

        if machine_lines.len() != 3 {
            return Err(ClawError::StringFormatError);
        }

        Ok(ClawMachine {
            button_a: Button::from_str(machine_lines[0])?,
            button_b: Button::from_str(machine_lines[1])?,
            prize: Prize::from_str(machine_lines[2])?,
        })
    }
}

#[derive(Debug)]
struct Button {
    x_inc: i32,
    y_inc: i32,
}

impl FromStr for Button {
    type Err = ButtonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.lines().count() != 1 {
            return Err(ButtonError::FormatError);
        }

        let incs: Vec<i32> = s
            .split_once(":")
            .unwrap()
            .1
            .split(", ")
            .map(|st| st.split_once("+").unwrap().1.parse().unwrap())
            .collect();

        Ok(Button {
            x_inc: incs[0],
            y_inc: incs[1],
        })
    }
}

#[derive(Debug)]
struct Prize {
    x: i32,
    y: i32,
}
impl FromStr for Prize {
    type Err = PrizeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.lines().count() != 1 {
            return Err(PrizeError::ParseError);
        }

        let loc: Vec<i32> = s
            .split_once(":")
            .unwrap()
            .1
            .split(", ")
            .map(|st| st.split_once("=").unwrap().1.parse().unwrap())
            .collect();

        Ok(Prize {
            x: loc[0],
            y: loc[1],
        })
    }
}

#[derive(Debug)]
enum PrizeError {
    ParseError,
}
impl Display for PrizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrizeError::ParseError => write!(f, "Error parsing string into prize"),
        }
    }
}
impl Error for PrizeError {}

#[derive(Debug)]
enum ButtonError {
    FormatError,
}

impl Display for ButtonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonError::FormatError => write!(f, "Format of button info is incorrect"),
        }
    }
}
impl Error for ButtonError {}

#[derive(Debug)]
enum ClawError {
    StringFormatError,
    Button(ButtonError),
    Prize(PrizeError),
}

impl From<PrizeError> for ClawError {
    fn from(value: PrizeError) -> Self {
        ClawError::Prize(value)
    }
}
impl From<ButtonError> for ClawError {
    fn from(value: ButtonError) -> Self {
        ClawError::Button(value)
    }
}

impl Display for ClawError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClawError::StringFormatError => write!(f, "Input string in invalid format"),
            ClawError::Prize(err) => write!(f, "{err}"),
            ClawError::Button(err) => write!(f, "{err}"),
        }
    }
}
impl Error for ClawError {}
