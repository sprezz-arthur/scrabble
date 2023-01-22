pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

pub fn bad_add(a: i32, b: i32) -> i32 {
    a + b + 1
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_adds() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    #[should_panic]
    fn it_does_not_work() {
        assert_eq!(bad_add(2, 2), 4);
    }
}
