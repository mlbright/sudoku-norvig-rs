use smallbitvec::SmallBitVec;

#[derive(Clone, Debug)]
pub struct Cell {
    bit_vector: SmallBitVec,
}

impl Cell {
    pub fn len(&self) -> usize {
        self.bit_vector
            .iter()
            .filter(|b| *b)
            .collect::<Vec<bool>>()
            .len()
    }

    pub fn new() -> Self {
        Cell {
            bit_vector: sbvec![true; 9],
        }
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for (i, b) in self.bit_vector.iter().enumerate() {
            if b {
                s.push_str(&i.to_string())
            }
        }
        s
    }

    pub fn possibilities(&self) -> Vec<u32> {
        self.bit_vector
            .iter()
            .enumerate()
            .filter_map(|(i, b)| if b { Some(i as u32) } else { None })
            .collect::<Vec<u32>>()
    }

    pub fn contains(&self, position: usize) -> bool {
        self.bit_vector[position]
    }

    pub fn remove(&mut self, position: usize) {
        self.bit_vector.set(position, false);
    }

    pub fn set(&mut self, position: usize) {
        self.bit_vector.set(position, true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let mut c = Cell::new();
        c.set(8);
        c.remove(4);
        assert_eq!(true, c.contains(8));
        assert_eq!(false, c.contains(4));
    }

    #[test]
    fn test_possibilities() {
        let mut c = Cell::new();
        c.remove(3);
        assert_eq!(c.possibilities(), vec![0, 1, 2, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_len() {
        let mut c = Cell::new();
        c.remove(3);
        c.remove(2);
        assert_eq!(c.len(), 7);
    }
}
