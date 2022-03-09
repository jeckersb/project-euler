#[derive(Default)]
pub struct TriangleIter {
    prev: u64,
    iter: u64,
}

impl TriangleIter {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Iterator for TriangleIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter += 1;
        self.prev += self.iter;
        Some(self.prev)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_iter() {
        let mut triangles = TriangleIter::new();

        assert_eq!(triangles.next(), Some(1));
        assert_eq!(triangles.next(), Some(3));
        assert_eq!(triangles.next(), Some(6));
        assert_eq!(triangles.next(), Some(10));
        assert_eq!(triangles.next(), Some(15));
        assert_eq!(triangles.next(), Some(21));
        assert_eq!(triangles.next(), Some(28));
        assert_eq!(triangles.next(), Some(36));
        assert_eq!(triangles.next(), Some(45));
        assert_eq!(triangles.next(), Some(55));
    }
}
