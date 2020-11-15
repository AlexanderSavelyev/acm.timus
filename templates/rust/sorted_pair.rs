use std::cmp::Ordering;
#[derive(PartialEq, Eq, Debug, Clone)]
struct UNode {
    a: u32,
    b: u32,
}

impl UNode {
    fn new(a: u32, b: u32) -> UNode {
        UNode { a: a, b: b }
    }
}

impl PartialOrd for UNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UNode {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.a == other.a {
            return self.b.cmp(&other.b);
        } else {
            return other.a.cmp(&self.a);
        }
    }
}