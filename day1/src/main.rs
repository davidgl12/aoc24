use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let splitted = contents.trim_end().split("\n");

    let mut list_a: Vec<i64> = vec![];
    let mut list_b: Vec<i64> = vec![];

    for spl in splitted {
        let nums: Vec<&str> = spl.split("   ").collect();

        let a: i64 = nums[0].parse().unwrap();
        let b: i64 = nums[1].parse().unwrap();

        list_a.push(a);
        list_b.push(b);
    }

    list_a.sort();
    list_b.sort();

    let mut result: i64 = 0;

    for i in 0..list_a.len() {
        let num = &list_a[i] - &list_b[i];
        result += num.abs();
    }

    println!("First answer: {result}"); // Answer to first challenge

    let mut similarity_score: i64 = 0;

    for a in &list_a {
        let mut appearances: i64 = 0;

        for b in &list_b {
            if a == b {
                appearances += 1;
            }
        }

        similarity_score = similarity_score + (a * appearances);
    }

    println!("Second answer (similarity_score): {similarity_score}")
}
