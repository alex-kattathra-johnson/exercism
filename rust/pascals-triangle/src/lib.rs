pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (1..=self.0).map(Self::row).collect()
    }

    fn row(n: u32) -> Vec<u32> {
        let mut r = vec![1];
        for p in 1..n {
            if let Some(&last) = r.last() {
                r.push(last * (n - p) / p);
            }
        }
        r
    }
}
