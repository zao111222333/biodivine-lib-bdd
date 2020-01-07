use crate::{Bdd, BddNode, BddPointer, BddVariable};
use std::io::{Read, Write};

impl Bdd {
    /// Write a BDD into the given writer using a simple string format.
    fn write_as_string(&self, output: &mut dyn Write) -> Result<(), std::io::Error> {
        write!(output, "|")?;
        for node in self.nodes() {
            write!(output, "{},{},{}|", node.var, node.low_link, node.high_link)?;
        }
        return Ok(());
    }

    /// Read a BDD from the given reader, assuming a simple string format.
    fn read_as_string(input: &mut dyn Read) -> Result<Bdd, String> {
        let mut data = String::new();
        lift_err(input.read_to_string(&mut data))?;
        let mut result = Vec::new();
        for node_string in data.split('|').filter(|s| !s.is_empty()) {
            let node_items: Vec<&str> = node_string.split(',').collect();
            let node = BddNode::mk_node(
                BddVariable(lift_err(node_items[0].parse::<u16>())?),
                BddPointer::from_index(lift_err(node_items[1].parse::<usize>())?),
                BddPointer::from_index(lift_err(node_items[2].parse::<usize>())?),
            );
            result.push(node);
        }
        return Ok(Bdd(result));
    }
    /// Write a BDD into the given writer using a simple little-endian binary encoding.
    pub fn write_as_bytes(&self, output: &mut dyn Write) -> Result<(), std::io::Error> {
        for node in self.nodes() {
            output.write(&node.var.to_le_bytes())?;
            output.write(&node.low_link.to_le_bytes())?;
            output.write(&node.high_link.to_le_bytes())?;
        }
        return Ok(());
    }

    /// Read a BDD from a given reader using a simple little-endian binary encoding.
    pub fn read_as_bytes(input: &mut dyn Read) -> Result<Bdd, std::io::Error> {
        let mut result = Vec::new();
        let mut buf = [0u8; 10];
        while input.read(&mut buf)? == 10 {
            result.push(BddNode::mk_node(
                BddVariable::from_le_bytes([buf[0], buf[1]]),
                BddPointer::from_le_bytes([buf[2], buf[3], buf[4], buf[5]]),
                BddPointer::from_le_bytes([buf[6], buf[7], buf[8], buf[9]]),
            ))
        }
        return Ok(Bdd(result));
    }

    /// Convert a BDD to a serialized string format.
    pub fn to_string(&self) -> String {
        let mut buffer: Vec<u8> = Vec::new();
        self.write_as_string(&mut buffer)
            .expect("Cannot write BDD to string.");
        return String::from_utf8(buffer).expect("Invalid UTF formatting in string.");
    }

    /// Read a BDD from a serialized string.
    pub fn from_string(bdd: &str) -> Bdd {
        return Bdd::read_as_string(&mut bdd.as_bytes()).expect("Invalid BDD string.");
    }

    /// Convert a BDD into a byte vector.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.write_as_bytes(&mut buffer)
            .expect("Error writing bytes.");
        return buffer;
    }

    /// Read a BDD from a byte vector.
    pub fn from_bytes(data: &mut &[u8]) -> Bdd {
        return Bdd::read_as_bytes(data).expect("Error reading bytes.");
    }
}

fn lift_err<T, E: ToString>(item: Result<T, E>) -> Result<T, String> {
    return item.map_err(|e| e.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{load_expected_results, mk_small_test_bdd};

    #[test]
    fn bdd_to_string() {
        let bdd = mk_small_test_bdd();
        let bdd_string = bdd.to_string();
        assert_eq!(load_expected_results("bdd_to_string.txt"), bdd_string);
    }

    #[test]
    fn bdd_from_string() {
        let data = load_expected_results("bdd_to_string.txt");
        let bdd = Bdd::from_string(&data);
        assert_eq!(mk_small_test_bdd(), bdd);
    }

    #[test]
    fn bdd_to_bytes() {
        let bdd = mk_small_test_bdd();
        let bdd_bytes = bdd.to_bytes();
        assert_eq!(bdd, Bdd::from_bytes(&mut &bdd_bytes[..]));
    }
}