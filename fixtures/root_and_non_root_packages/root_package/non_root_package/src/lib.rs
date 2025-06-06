pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_in_non_root_package() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
