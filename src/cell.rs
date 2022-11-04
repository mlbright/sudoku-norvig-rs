use arrayvec::ArrayVec;

#[derive(Clone, Debug)]
pub struct Cell(u16);

impl Default for Cell {
    fn default() -> Self {
        Self::new()
    }
}

impl Cell {
    pub fn new() -> Self {
        Cell(0b0000_0011_1111_1110u16)
    }

    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn possibilities(&self) -> ArrayVec<usize, 9> {
        let mut indices = ArrayVec::<_, 9>::new();
        let mut t = self.0;
        while t > 0 {
            let index = t.trailing_zeros();
            indices.push(index as usize);
            t &= !(1 << index);
        }
        indices
    }

    pub fn first(&self) -> Option<usize> {
        match self.possibilities().len() {
            0 => None,
            _ => Some(self.possibilities()[0]),
        }
    }

    pub fn contains(&self, position: usize) -> bool {
        let mask = 1<<position;
        self.0 & mask != 0
    }

    pub fn remove(&mut self, position: usize) {
        self.0 &= !(1 << position);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let mut c = Cell::new();
        c.remove(4);
        assert!(c.contains(1));
        assert!(c.contains(8));
        assert!(!c.contains(4));
        c.remove(8);
        assert!(!c.contains(8));
    }

    #[test]
    fn test_possibilities() {
        let mut c = Cell::new();
        c.remove(9);
        let mut array = ArrayVec::<_, 9>::new();
        array.push(1);
        array.push(2);
        array.push(3);
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
