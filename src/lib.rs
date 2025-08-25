pub mod Station;
pub mod Division;
pub mod Train;
pub mod Industry;
pub mod CarType;
pub mod Owner;
pub mod Car;
pub mod SwitchList;



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
