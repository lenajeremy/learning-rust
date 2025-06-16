pub fn abs<T: Into<i64>>(value: T) -> i64 {
    let value = value.into();
    if value < 0 {
        -value
    } else {
        value
    }
}