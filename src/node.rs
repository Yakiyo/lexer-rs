struct Node {
    start: usize,
    end: usize,
}

impl Node {
    fn new(start: usize, end: usize) -> Self {
        Node {
            start,
            end
        }
    }
}