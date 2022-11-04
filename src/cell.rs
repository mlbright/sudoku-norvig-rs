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

    pub fn first(&self) -> usize {
        let n = self.0.trailing_zeros();
        match n {
            0 => 0,
            _ => (n as u16).into(),
        }
    }

    pub fn contains(&self, position: usize) -> bool {
        let mask = 1 << position;
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
    fn test_len() {
        let mut c = Cell::new();
        c.remove(3);
        c.remove(2);
        assert_eq!(c.len(), 7);
    }
}
