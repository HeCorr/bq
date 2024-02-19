/// Returns `Yes` if input is true, `No` otherwise.
pub fn bool_fmt(input: bool) -> &'static str {
    if input {
        "Yes"
    } else {
        "No"
    }
}
