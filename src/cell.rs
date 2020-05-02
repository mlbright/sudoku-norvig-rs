use smallbitvec::SmallBitVec;
use arrayvec::ArrayVec;

#[derive(Clone, Debug)]
pub struct Cell {
    bit_vector: SmallBitVec,
    length: usize,
}

impl Cell {
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn new() -> Self {
        Cell {
            bit_vector: sbvec![true; 9],
            length: 9,
        }
    }

    pub fn possibilities(&self) -> ArrayVec<[usize;9]> {
        self.bit_vector
            .iter()
            .enumerate()
            .filter(|(_i, b)| *b)
            .map(|(i, _b)| i)
            .collect::<_>()
    }

    pub fn possibilities_except(&self, exception: usize) -> ArrayVec<[usize;9]> {
        self.bit_vector
            .iter()
            .enumerate()
            .filter(|(_i, b)| *b)
            .map(|(i, _b)| i)
            .filter(|i| *i != exception)
            .collect::<ArrayVec<[usize;9]>>()
    }

    pub fn first(&self) -> Option<usize> {
        for (i, b) in self.bit_vector.iter().enumerate() {
            if b {
                return Some(i);
            }
        }
        None
    }

    pub fn contains(&self, position: usize) -> bool {
        self.bit_vector[position]
    }

    pub fn remove(&mut self, position: usize) {
        self.bit_vector.set(position, false);
        self.length -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let mut c = Cell::new();
        c.remove(4);
        assert_eq!(true, c.contains(8));
        assert_eq!(false, c.contains(4));
    }

    #[test]
    fn test_possibilities() {
        let mut c = Cell::new();
        c.remove(3);
        let mut array = ArrayVec::<[_; 9]>::new();
        array.push(0);
        array.push(1);
        array.push(2);
        array.push(4);
        array.push(5);
        array.push(6);
        array.push(7);
        array.push(8);
        assert_eq!(c.possibilities(), array);
    }

    #[test]
    fn test_len() {
        let mut c = Cell::new();
        c.remove(3);
        c.remove(2);
        assert_eq!(c.len(), 7);
    }
}
