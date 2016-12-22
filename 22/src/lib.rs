use std::str::FromStr;
use std::error::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub used_tb: u32,
    pub free_tb: u32,
}

pub fn parse_size(s: &str) -> Result<u32, Box<Error>> {
    let size: u32 = s[0..s.len()-1].parse()?;
    assert!(s.ends_with('T'));
    Ok(size)
}

impl FromStr for Node {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace().skip(1);
        let size: u32 = parse_size((tokens.next().ok_or("Not enough token")?))?;
        let used: u32 = parse_size((tokens.next().ok_or("Not enough token")?))?;
        let free: u32 = parse_size((tokens.next().ok_or("Not enough token")?))?;

        assert_eq!(used + free, size);
        Ok(Node{
            used_tb: used,
            free_tb: free,
        })
    }
}

pub fn is_viable_pair(left: &Node, right: &Node) -> bool {
    left.used_tb != 0 && left.used_tb <= right.free_tb
}

pub fn puzzle(input: &str) -> usize {
    let nodes: Vec<Node> = input.lines().skip(2).map(|v| {
        v.parse().expect(&format!("Failed to parse line {}", v))
    }).collect();

    let mut number_of_viable_pairs: usize = 0;
    for node in nodes.iter() {
        number_of_viable_pairs += nodes.iter().filter(|b| is_viable_pair(node, b)).count();
        if is_viable_pair(node, node) {
            number_of_viable_pairs -= 1
        }
    }

    number_of_viable_pairs
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_node() {
        let input = "/dev/grid/node-x0-y0     89T   67T    22T   75%";
        let node: Node = input.parse().unwrap();
        assert_eq!(node, Node{used_tb: 67, free_tb: 22});
    }
}
