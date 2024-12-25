use std::{error::Error, fmt::Display, fs, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let machines = fs::read_to_string("input.txt")?
        .split("\n\n")
        .map(|block| ClawMachine::from_str(block))
        .collect::<Result<Vec<_>, _>>()?;

    println!(
        "{}",
        machines
            .iter()
            .filter_map(|m| m.get_solution())
            .map(|(p_a, p_b)| p_a * 3 + p_b * 1)
            .sum::<i64>()
    );

    Ok(())
}

#[derive(Debug)]
struct ClawMachine {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

impl ClawMachine {
    fn get_solution(&self) -> Option<(i64, i64)> {
        // Get everything as f64 to make sure calculations don't mess up
        let x = self.prize.x as f64;
        let y = self.prize.y as f64;
        let a_x = self.button_a.x_inc as f64;
        let a_y = self.button_a.y_inc as f64;
        let b_x = self.button_b.x_inc as f64;
        let b_y = self.button_b.y_inc as f64;

        // Calculations
        let p_b = (x - ((a_x * y) / a_y)) / (b_x - ((a_x * b_y) / a_y));
        let p_a = (y - p_b * b_y) / a_y;

        // Check if the solution actually works (accounting for floating point error)
        if p_a.round() * a_x + p_b.round() * b_x == x && p_a.round() * a_y + p_b.round() * b_y == y
        {
            Some((p_a.round() as i64, p_b.round() as i64))
        } else {
            None
        }
    }
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
    x_inc: i64,
    y_inc: i64,
}

impl FromStr for Button {
    type Err = ButtonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.lines().count() != 1 {
            return Err(ButtonError::FormatError);
        }

        let incs: Vec<i64> = s
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
    x: i64,
    y: i64,
}
impl FromStr for Prize {
    type Err = PrizeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.lines().count() != 1 {
            return Err(PrizeError::ParseError);
        }

        let loc: Vec<i64> = s
            .split_once(":")
            .unwrap()
            .1
            .split(", ")
            .map(|st| st.split_once("=").unwrap().1.parse().unwrap())
            .collect();

        Ok(Prize {
            x: loc[0] + 10000000000000,
            y: loc[1] + 10000000000000,
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
