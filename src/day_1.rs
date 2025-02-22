use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn a() {
    prina();
}

pub fn day_1_pt_1() -> () {
    println!("day 1 part 1");
    let mut input_vec: Vec<String> = Vec::new();
    let _a = read_text("input_1.txt", &mut input_vec);
    println("ABCD");
    let mut cols = create_cols(&mut input_vec);
    let _sum = cln_sum(&mut cols.0, &mut cols.1);
    let counts = get_count(&mut cols.0, &mut cols.1);
    println!("{}", counts);
}
fn read_text<P: AsRef<Path>>(
    filename: P,
    input_vector: &mut Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        input_vector.push(line?);
    }
    Ok(())
}
fn create_cols(input_vector: &mut Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut left_col: Vec<i32> = Vec::new();
    let mut right_col: Vec<i32> = Vec::new();
    for i in 0..input_vector.len() {
        let split_str: Vec<&str> = input_vector[i].split(' ').collect();
        left_col.push(split_str[0].parse().expect("X"));
        right_col.push(split_str[3].parse().expect("X"));
    }
    left_col.sort();
    right_col.sort();
    (left_col, right_col)
}
fn cln_sum(left_vec: &mut Vec<i32>, right_vec: &mut Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in 0..left_vec.len() {
        let temp_sum = left_vec[i] - right_vec[i];
        if temp_sum < 0 {
            sum += temp_sum * -1;
        } else {
            sum += temp_sum;
        }
    }

    sum
}
//solved part 1, answer  = 1834060
fn get_count(left_vec: &mut Vec<i32>, right_vec: &mut Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in 0..left_vec.len() {
        let occurance = right_vec.iter().filter(|&x| *x == left_vec[i]).count();
        let value = left_vec[i] * occurance as i32;
        sum += value;
    }
    sum
}
//solved part 2, answer  = 21607792
