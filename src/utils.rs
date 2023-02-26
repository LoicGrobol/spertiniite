

struct NgramIter<'a, T> {
    wrapped: Vec<T>,
    n,
    current_indix,
}
