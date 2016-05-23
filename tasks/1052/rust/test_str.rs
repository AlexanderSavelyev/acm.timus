use std::collections::HashSet;

struct TestStr {
    pub lines: HashSet<i32>
}

impl TestStr {
    pub fn new() -> TestStr {
        TestStr { lines: HashSet::new() }
    }

    pub fn do_smth (&mut self) {
        self.lines.insert(1i32);
    }
}

fn main() {
    let mut test_str = TestStr::new();
    test_str.do_smth();
    println!("{}", 1);
}
