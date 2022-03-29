// yes, input.into_iter().map(function).collect() would work
// please read the README for more info
pub fn map<T, O, F>(input: Vec<T>, mut function: F) -> Vec<O>
where
    F: FnMut(T) -> O,
{
    let mut res = Vec::with_capacity(input.len());
    for value in input {
        res.push(function(value));
    }
    res
}
