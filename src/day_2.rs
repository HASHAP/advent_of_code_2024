use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn day_2() -> () {
    println!("day 2");
    let mut input_vec: Vec<String> = Vec::new();
    let _a = read_text("input_2.txt", &mut input_vec);
    let mut vec_2d = converter(&mut input_vec);
    println!("{}", vec_2d.len());
    check_validity(&mut vec_2d);
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
fn converter(input_vector: &mut Vec<String>) -> Vec<Vec<i16>> {
    let mut vec_2d: Vec<Vec<i16>> = Vec::new();
    for i in 0..input_vector.len() {
        let split_str: Vec<&str> = input_vector[i].split(' ').collect();
        let temp_vec: Vec<i16> = split_str.iter().map(|x| x.parse().unwrap()).collect();
        vec_2d.push(temp_vec);
    }
    vec_2d
}
fn check_order(in_vec: &mut Vec<i16>) -> i8 {
    let mut temp_sort = in_vec.clone();
    let mut temp_usort = in_vec.clone();
    temp_sort.sort();
    temp_usort.sort_by(|a, b| b.cmp(a));
    if &temp_sort == in_vec {
        return 1;
    } else if &temp_usort == in_vec {
        return 2;
    }
    0
}
fn check_validity(in_vec: &mut Vec<Vec<i16>>) -> () {
    let mut valid_vec: Vec<bool> = Vec::new();
    for i in 0..in_vec.len() {
        let value = check_order(&mut in_vec[i]);
        if value > 0 {
           valid_vec.push(check_distance(&mut in_vec[i], value));
        } else {
            valid_vec.push(false);
        }
    }
    let tr_count = valid_vec.iter().filter(|&x| *x == true).count();
    println!("{}", tr_count);
    println!("{}", valid_vec.len());
}
fn check_distance(in_vec: &mut Vec<i16>, sort: i8) -> bool {
    if sort == 1_i8 {
        let common_diff = in_vec[1] - in_vec[0];
        for i in 1..in_vec.len() - 1 {
            if in_vec[i + 1] - in_vec[i] != common_diff {
                return false;
            }
        }
    } else {
        let common_diff = in_vec[0] - in_vec[1];
        for i in 1..in_vec.len() - 1 {
            if in_vec[i] - in_vec[i+1] != common_diff {
                return false;
            }
        }
    }
    println!("{:?}", in_vec);
    true
}
