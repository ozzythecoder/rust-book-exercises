use std::collections::HashMap;

fn main() {

    // Challenge: Given a list of integers, use a vector 
    // and return the median (when sorted, the value in the middle position)
    // and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    let ints = vec![1, 1, 1, 3, 4, 4, 5, 7];
    println!("{:?}", ints);

    let mut sum = 0;
    for i in &ints {
        sum += i;
    }
    let mean_average = sum / ints.len();
    let mut map = HashMap::new();

    for i in ints {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let values_by_occurence = Vec::from_iter(map.iter());

    let mut mode: usize = 1;
    let mut max_occurences = 0;

    for (val, occurences) in values_by_occurence {
        if occurences > &max_occurences {
            mode = *val;
            max_occurences = *occurences;
        }
    }

    println!("Sum is {}", sum);
    println!("Mean average is {}", mean_average);
    println!("Mode is {}", mode);
}
