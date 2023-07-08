use std::ops::Range;

/// `Vec` whose elements have weight.
///
/// You can choose an element from `WeightedVec` depends on their weight.
#[derive(Debug, Clone)]
pub struct WeightedVec<T> {
    pub elements: Vec<T>,
    pub ranges: Vec<Range<usize>>,
    pub next: usize,
}

impl<T> WeightedVec<T> {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            ranges: Vec::new(),
            next: 0,
        }
    }

    /// Appends an element and his weight
    pub fn push(&mut self, weight: usize, element: T) {
        self.elements.push(element);
        self.ranges.push(self.next..self.next+weight);
        self.next += weight;
    }

    /// Chooses an element from collections depends on weights of elements.
    ///
    /// Accepts `num` in range 0..len, where len is collection's lenght
    pub fn choose(&self, num: usize) -> Option<&T> {
        for (i, range) in self.ranges.iter().enumerate() {
            if range.contains(&num) {
                return self.elements.get(i);
            }
        }
        None
    }
    
    /// Returns count of collection's elements
    pub fn len(&self) -> usize {
        self.elements.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn choice1() {
        let mut list: WeightedVec<usize> = WeightedVec::new();
        list.push(10, 0);
        list.push(10, 1);
        list.push(10, 2);

        assert_eq!(list.choose(0), Some(&0));
        assert_eq!(list.choose(9), Some(&0));
        assert_eq!(list.choose(10), Some(&1));
        assert_eq!(list.choose(19), Some(&1));
        assert_eq!(list.choose(20), Some(&2));
        assert_eq!(list.choose(29), Some(&2));
        assert_eq!(list.choose(30), None);
    }
}
