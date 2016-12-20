use std::str::FromStr;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IpRange {
    pub start: u32,
    pub end: u32
}

impl Display for IpRange {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.start, self.end)
    }
}

impl FromStr for IpRange {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let range = IpRange {
            start: parts.next().expect("Not enough tokens").parse()?,
            end: parts.next().expect("Not enough tokens").parse()?
        };
        assert!(range.start < range.end);
        Ok(range)
    }
}

impl IpRange {
    pub fn is_ip_in_range(&self, ip: u32) -> bool {
        ip >= self.start && ip <= self.end
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Blacklist {
    list: Vec<IpRange>
}

impl Display for Blacklist {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for ip_range in &self.list {
            writeln!(f, "{}", ip_range)?;
        }
        Ok(())
    }
}

impl FromStr for Blacklist {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut list: Vec<IpRange> = s.lines().enumerate().map(|(line_number, line)| {
            IpRange::from_str(line)
                .expect(&format!("Failed to parse line {}: {}", line_number, line))
        }).collect();
        list.sort();
        Ok(Blacklist{list: list})
    }
}

impl Blacklist {
    pub fn is_blocked(&self, ip: u32) -> bool {
        self.list.iter().any(|v| v.is_ip_in_range(ip))
    }

    pub fn number_of_allowed_ips(&self) -> usize {
        (0..4294967295).filter(|v| !self.is_blocked(*v)).count()
    }

    pub fn lowest_ip(&self) -> u32 {
        let lowest = (0..4294967295).filter(|v| !self.is_blocked(*v)).next();
        lowest.expect("no unblocked IP found")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_line() {
        let input = "5-8";
        let range: IpRange = input.parse().unwrap();
        assert_eq!(range, IpRange{start: 5, end: 8});
    }

    #[test]
    fn parse_sample_input() {
        let input = "5-8\n0-2\n4-7";
        let blacklist: Blacklist = input.parse().unwrap();
        assert_eq!(blacklist, Blacklist{list: vec![
            IpRange{start: 0, end: 2},
            IpRange{start: 4, end: 7},
            IpRange{start: 5, end: 8},
        ]});
    }

    #[test]
    fn find_lowest_in_sample_input() {
        let input = "5-8\n0-2\n4-7";
        let blacklist: Blacklist = input.parse().unwrap();
        let lowest = blacklist.lowest_ip();
        assert_eq!(lowest, 3);
    }
}
