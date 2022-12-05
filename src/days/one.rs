use std::fs;

fn extract_calorie_packs(string: &str) -> Vec<Vec<i32>> {
    let mut packs_vector = vec![];
    let pattern = "\n\n";
    let packs_iterator = string.split(pattern);
    for pack in packs_iterator {
        let mut pack_vector = vec![];
        let pack_lines = pack.split("\n");
        for pack_line in pack_lines {
            let calorie = pack_line.parse::<i32>().unwrap();
            pack_vector.push(calorie)
        }
        packs_vector.push(pack_vector)
    }
    return packs_vector;
}

fn sum_up_calories(vector: Vec<Vec<i32>>) -> Vec<i32> {
    let mut sum_vector = vec![];
    for sub_vector in vector {
        let sum = sub_vector.iter().sum();
        sum_vector.push(sum);
    }
    return sum_vector;
}

fn get_max_calories(vector: Vec<i32>) -> i32 {
    let max = vector.iter().max().unwrap();
    return *max;
}

pub fn get_answer() -> i32 {
    let content = fs::read_to_string("resources/one.txt").expect("Unable to read file");
    let content_as_str = content.as_str();
    let packs = extract_calorie_packs(content_as_str);
    let calories = sum_up_calories(packs);
    let max = get_max_calories(calories);
    return max;
}

#[cfg(test)]
mod tests {
    use super::{extract_calorie_packs, get_max_calories, sum_up_calories};

    #[test]
    fn extract_calories_pack_test() {
        let first_pack = vec![1000, 1000];
        let second_pack = vec![500, 250];
        let exprected_packs = vec![first_pack, second_pack];

        let string = "1000\n1000\n\n500\n250";
        let actual_packs = extract_calorie_packs(string);

        assert_eq!(exprected_packs, actual_packs)
    }

    #[test]
    fn sum_up_calories_test() {
        let exprected_packs = vec![2000, 750];

        let first_pack = vec![1000, 1000];
        let second_pack = vec![500, 250];
        let calories_packs = vec![first_pack, second_pack];
        let actual_packs = sum_up_calories(calories_packs);

        assert_eq!(exprected_packs, actual_packs)
    }

    #[test]
    fn get_max_calories_test() {
        let expected_sum = 2000;

        let calories_sums = vec![2000, 750];
        let actual_sum = get_max_calories(calories_sums);

        assert_eq!(expected_sum, actual_sum)
    }
}
