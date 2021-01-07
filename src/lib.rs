/// The trait Last. This is designed for vectors.
trait Last<T> {
    fn last(&self) -> &T;
}

impl<T> Last<T> for Vec<T> {
    fn last(&self) -> &T {
        &self[self.len() - 1]
    }
}
