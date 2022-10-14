pub struct BitCartesianN {
    n: u32,
    counter: u32,
}

impl BitCartesianN {
    fn new(n: u32) -> Self {
        Self {
            n,
            counter: 0,
        }
    }
}

impl Iterator for BitCartesianN {
    type Item = Vec<bool>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter >= 2u32.pow(self.n) {
            return None;
        } else if self.n == 0 {
            return None;
        }
        
        let mut bits = Vec::new();
        
        let mut extract = 1 << self.n - 1;
        
        for _ in 0..self.n {
            println!("{:b} {:b} {}", self.counter, extract, self.counter & extract);
            
            bits.push(self.counter & extract >= 1);
            extract = extract >> 1;
        }
        
        self.counter += 1;
        
        Some(bits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bitcartesiann_0() {
        let a = BitCartesianN::new(0);
        let result = a.collect::<Vec<_>>();
        assert!(result.is_empty());
    }
    
    #[test]
    fn test_bitcartesiann_1() {
        let a = BitCartesianN::new(1);
        let result = a.collect::<Vec<_>>();
        assert_eq!(result, vec![
            vec![false],
            vec![true],
        ]);
    }
    
    #[test]
    fn test_bitcartesiann_3() {
        let a = BitCartesianN::new(3);
        let result = a.collect::<Vec<_>>();
        assert_eq!(result, vec![
            vec![false, false, false],
            vec![false, false, true],
            vec![false, true, false],
            vec![false, true, true],
            vec![true, false, false],
            vec![true, false, true],
            vec![true, true, false],
            vec![true, true, true],
        ]);
    }
}