use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
enum FizzBuzz {
    FizzBuzz,
    Fizz,
    Buzz,
    Number(u32),
}

impl From<u32> for FizzBuzz {
    fn from(number: u32) -> Self {
        match (number % 3, number % 5) {
            (0, 0) => FizzBuzz::FizzBuzz,
            (0, _) => FizzBuzz::Fizz,
            (_, 0) => FizzBuzz::Buzz,
            (_, _) => FizzBuzz::Number(number)
        }
    }
}

impl Display for FizzBuzz {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            FizzBuzz::Number(n) => write!(f, "{}", n),
            _ => write!(f, "{:?}", self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_one_when_number_is_one() {
        assert_eq!(FizzBuzz::from(1).to_string(), "1");
    }

    #[test]
    fn should_be_two_when_number_is_two() {
        assert_eq!(FizzBuzz::from(2).to_string(), "2");
    }

    #[test]
    fn should_be_four_when_number_is_four() {
        assert_eq!(FizzBuzz::from(4).to_string(), "4");
    }

    #[test]
    fn should_be_fizz_when_number_is_three() {
        assert_eq!(FizzBuzz::from(3).to_string(), "Fizz");
    }

    #[test]
    fn should_be_fizz_when_number_is_six() {
        assert_eq!(FizzBuzz::from(6).to_string(), "Fizz");
    }

    #[test]
    fn should_be_fizz_when_number_is_nine() {
        assert_eq!(FizzBuzz::from(9).to_string(), "Fizz");
    }

    #[test]
    fn should_be_buzz_when_number_is_five() {
        assert_eq!(FizzBuzz::from(5).to_string(), "Buzz");
    }

    #[test]
    fn should_be_buzz_when_number_is_ten() {
        assert_eq!(FizzBuzz::from(10).to_string(), "Buzz");
    }

    #[test]
    fn should_be_buzz_when_number_is_twenty() {
        assert_eq!(FizzBuzz::from(20).to_string(), "Buzz");
    }

    #[test]
    fn should_be_fizzbuzz_when_number_is_fifteen() {
        assert_eq!(FizzBuzz::from(15).to_string(), "FizzBuzz");
    }

    #[test]
    fn should_be_fizzbuzz_when_number_is_thirty() {
        assert_eq!(FizzBuzz::from(30).to_string(), "FizzBuzz");
    }

    #[test]
    fn should_be_fizzbuzz_when_number_is_fourty_five() {
        assert_eq!(FizzBuzz::from(45).to_string(), "FizzBuzz");
    }
}
