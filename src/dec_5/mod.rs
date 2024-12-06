use crate::read_file;
use std::collections::HashMap;

pub fn day_five() {
    let input = read_file("src/dec_5/input.txt".to_string());
    let mut input = input.trim().split("\n\n");

    let rules_unparsed = input.next().unwrap().split("\n");

    let mut rules: HashMap<String, Vec<String>> = HashMap::new();

    for rule in rules_unparsed {
        let mut rule = rule.split("|");
        let (pre, post) = (rule.next().unwrap(), rule.next().unwrap());

        // We point pre to a list of post; that is, this vector is a list of numbers which may not proceed the current number
        if rules.contains_key(&post.to_string()) {
            rules
                .get_mut(&post.to_string())
                .unwrap()
                .push(pre.to_string());
        } else {
            rules.insert(post.to_string(), vec![pre.to_string()]);
        }
    }

    let updates = input
        .next()
        .unwrap()
        .split("\n")
        .map(|update| update.split(",").map(|u| u.to_string()).collect())
        .collect::<Vec<Vec<String>>>();

    let mut middle_sum = 0;
    let mut reordered_middle_sum = 0;

    'updates: for update in updates {
        let middle = get_middle(&update);
        let mut pages_disallowed = vec![];

        for page in &update {
            if pages_disallowed.contains(&page) {
                let mut sorted = update.clone();
                sort_list(&mut sorted, &rules, 0, update.len() as i32 - 1);
                reordered_middle_sum += get_middle(&sorted).parse::<i32>().unwrap();
                continue 'updates;
            }

            if let Some(future_page) = rules.get(page) {
                for page in future_page {
                    pages_disallowed.push(page);
                }
            }
        }

        middle_sum += middle.parse::<i32>().unwrap();
    }

    println!("Middle sum: {}", middle_sum);
    println!("Reordered middle sum: {}", reordered_middle_sum);
}

fn sort_list(
    list: &mut Vec<String>,
    rules: &HashMap<String, Vec<String>>,
    low: i32,
    high: i32,
) -> Vec<String> {
    // Our hashmap is a hashmap of numbers which may not proceed their key
    // I.e. 2 -> 1, 10 means that 1 and 10 are less than 2
    // This constitutes a total ordering, so we can leverage quicksort
    if low < high {
        let pi = partition(list, rules, low, high);

        sort_list(list, rules, low, pi - 1);
        sort_list(list, rules, pi + 1, high);
    }

    list.to_vec()
}

fn partition(
    list: &mut Vec<String>,
    rules: &HashMap<String, Vec<String>>,
    low: i32,
    high: i32,
) -> i32 {
    // Our pivot is the parsed value of the last element
    let pivot = list[high as usize].parse::<i32>().unwrap();
    let mut i = low - 1;

    for j in low..high {
        let current = list[j as usize].parse::<i32>().unwrap();
        // All the numbers in the 'rule' vector are considered less than the pivot
        if let Some(rule) = rules.get(&pivot.to_string()) {
            if rule.contains(&current.to_string()) {
                i += 1;
                list.swap(i as usize, j as usize);
            }
        }

        // Otherwise we don't swap
    }

    list.swap((i + 1) as usize, high as usize);
    i + 1
}

fn get_middle(list: &Vec<String>) -> String {
    let middle = list.len() / 2;
    list[middle].to_string()
}
