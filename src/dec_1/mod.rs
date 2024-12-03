// Read text from file src/one/input.txt
use crate::read_file;
use std::collections::HashMap;

pub fn day_one() {
    let input = read_file("src/dec_1/input.txt".to_string());
    let input = input.trim();

    let mut list_one: Vec<i32> = vec![];
    let mut list_two: Vec<i32> = vec![];

    let mut second_count: HashMap<i32, i32> = HashMap::new();

    input.split("\n").for_each(|line| {
        let mut line = line.split("   ");

        let el_one = line.next().unwrap().parse().unwrap();
        let el_two = line.next().unwrap().parse().unwrap();

        list_one.push(el_one);
        list_two.push(el_two);

        *second_count.entry(el_two).or_default() += 1;
    });

    list_one.sort();
    list_two.sort();

    let diff = list_one
        .iter()
        .zip(list_two.iter())
        .map(|(a, b)| (a - b).abs())
        .collect::<Vec<i32>>();
    let similarity = list_one
        .iter()
        .map(|a| second_count.get(a).unwrap_or(&0) * a)
        .collect::<Vec<i32>>();

    let sum: i32 = diff.iter().sum();
    let similarity: i32 = similarity.iter().sum();

    println!("Sum: {}", sum);
    println!("Similarity: {}", similarity);
}
