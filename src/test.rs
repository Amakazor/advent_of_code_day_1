#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use crate::{extract_caloric_sums, get_max_calories, sum_max_calories};

    #[test]
    fn test_extract_caloric_sums(){
        let data = read_to_string("test-data.txt").expect("Error reading file");

        assert_eq!(extract_caloric_sums(data.as_str()), vec![4000, 6000, 10000, 11000, 24000])
    }

    #[test]
    fn test_get_max_calories(){
        let data = read_to_string("test-data.txt").expect("Error reading file");

        assert_eq!(get_max_calories(data.as_str()), 24000)
    }

    #[test]
    fn test_sum_max_calories() {
        let data = read_to_string("test-data.txt").expect("Error reading file");

        assert_eq!(sum_max_calories(data.as_str(), 3), 45000)
    }
}