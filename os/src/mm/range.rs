use core::fmt::Debug;

#[derive(Copy, Clone)]
pub struct Range<T: Eq + Copy + Step + PartialOrd + Debug>
{
    pub l: T,
    pub r: T,
}

impl<T> Range<T>
where
    T: Eq + Copy + Step + PartialOrd + Debug,
{
    pub fn new(start: T, end: T) -> Self {
        assert!(start <= end, "start {:?} > end {:?}!", start, end);
        Self { l: start, r: end }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter{
            cur: self.l, 
            end: self.r
        }
    }
}

pub trait Step {
    fn step(&mut self);
}

pub struct Iter<T: Eq + Copy + Step + PartialOrd + Debug> {
    cur: T,
    end: T,
}

impl<T> Iterator for Iter<T>
where
    T: Eq + Copy + Step + PartialOrd + Debug,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.end {
            None
        } else {
            let t = self.cur;
            self.cur.step();
            Some(t)
        }
    }
}

