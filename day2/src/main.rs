
#[derive(PartialEq, Debug)]
struct PasswordPolicy {
    byte: u8,
    positions: [usize; 2],
}


impl PasswordPolicy {
    fn is_valid(&self, password: &str) -> bool {
        self.positions
            .iter()
            .copied()
            .filter(|&index | password.as_bytes()[index] == self.byte)
            .count()
            == 1
    }
}


fn parse_line(s: &str) -> anyhow::Result<(PasswordPolicy, &str)> {

    peg::parser! {
        grammar parser() for str {
            rule number() -> usize
                = n:$(['0'..='9']+) { n.parse().unwrap() }
            
            /// Positions are 1-based indices in the input
            rule position() -> usize
                = n:number() { n - 1 }

            rule positions() -> [usize; 2]
                = first:position() "-" second:position() { [first, second] }
            
            rule byte() -> u8
                = letter:$(['a'..='z']) { letter.as_bytes()[0] }
            
            rule password() -> &'input str
                = letters:$([_]*) { letters }
            
            pub(crate) rule line() -> (PasswordPolicy, &'input str)
                = positions:positions() " " byte:byte() ": " password:password() {
                    (PasswordPolicy { positions, byte }, password)
                }
        }
    }

    Ok(parser::line(s)?)
}

fn main() -> anyhow::Result<()> {
    let count = include_str!("input.txt")
        .lines()
        .map(parse_line)
        .map(Result::unwrap)
        .filter(|(policy, password)| policy.is_valid(password))
        .count();
    
    println!("{} passwords are valid", count);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::PasswordPolicy;
    use super::parse_line;

    #[test]
    fn test_is_valid() {
        let pp = PasswordPolicy{
            positions: [0, 2],
            byte: b'a',
        };

        assert_eq!(pp.is_valid("abcde"), true, "'a' in position 1");
        assert_eq!(pp.is_valid("bcade"), true, "'a' in position 3");
        assert_eq!(pp.is_valid("food"), false, "no 'a' whatsoever");
        assert_eq!(pp.is_valid("abacus"), false, "'a' in both positions");
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_line("1-3 a: banana").unwrap(),
            (
                PasswordPolicy {
                    positions: [0, 2],
                    byte: b'a',
                },
                "banana"
            )
        );
    }
}