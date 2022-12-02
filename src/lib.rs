pub fn process_part1(_input: &str) -> String {
    "works".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("works".to_string(), process_part1("input"));
    }
}
