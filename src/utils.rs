/// Check if the first elements of `data` matches `elements`
pub fn first<E, T: PartialEq<E>>(data: &[T], elements: &[E]) -> bool {
    if data.len() < elements.len() {
        false
    } else {
        &data[0..elements.len()] == elements
    }
}
