use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let list = input.split("\n").map(|item| {
        if item == "" {
            return -1;
        }

        return item.parse::<i32>().unwrap();
    });

    let mut sum_list: Vec<i32> = vec![];
    let mut current_sum: i32 = 0;

    list.for_each(|item| {
        if item == -1 {
            sum_list.push(current_sum);
            current_sum = 0;
        } else {
            current_sum += item;
        }
    });

    sum_list.sort();

    let max = sum_list.last();
    let max_three: i32 = sum_list.split_at(sum_list.len() - 3).1.iter().sum();

    println!("{:?} - {:?}", max, max_three);
}
