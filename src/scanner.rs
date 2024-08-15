pub struct Scanner {
    start: Vec<char>,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn init(source: String) -> Self {
        Scanner {
            start: source.chars().collect::<Vec<char>>(),
            current: 0,
            line: 1,
        }
    }
}
