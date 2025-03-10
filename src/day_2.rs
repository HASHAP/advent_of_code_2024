use std::{
    error::Error,
    fs::File,
    i8,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn d_2_p_2() {
    println!("day 2 part 1");
    let mut in_vec: Vec<String> = Vec::new();
    let _a = read_text("input_2.txt", &mut in_vec);
    let num_vec: Vec<Vec<i32>> = create_num_vec(&mut in_vec);
    let _b = validity(num_vec);
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
fn create_num_vec(str_vec: &mut Vec<String>) -> Vec<Vec<i32>> {
    let mut num_vec: Vec<Vec<i32>> = Vec::new();
    for i in 0..str_vec.len() {
        let split_vec: Vec<&str> = str_vec[i].split(' ').collect();
        let temp_vec: Vec<i32> = split_vec.iter().map(|x| x.parse().unwrap()).collect();
        num_vec.push(temp_vec);
    }
    num_vec
}
fn check_order(num_vec: &mut Vec<Vec<i32>>) -> Vec<i8> {
    let mut sort_vec: Vec<i8> = Vec::new();
    for i in 0..num_vec.len() {
        let mut asc_vec: Vec<i32> = num_vec[i].clone();
        let mut des_vec: Vec<i32> = num_vec[i].clone();
        des_vec.sort_by(|a, b| b.cmp(a));
        asc_vec.sort();
        if num_vec[i] == asc_vec {
            sort_vec.push(0);
        } else if num_vec[i] == des_vec {
            sort_vec.push(1);
        } else {
            sort_vec.push(2);
        }
    }
    return sort_vec;
}
fn check_distance(num_vec: &Vec<i32>, order: i8) -> bool {
    match order {
        1 => {
            for i in 0..(num_vec.len() - 1) {
                if num_vec[i] - num_vec[i + 1] > 3 {
                    return false;
                }else if num_vec[i] -num_vec[i+1] < 1{
                    return false
                }
            }
        }
        0 => {
            for i in 0..(num_vec.len() - 1) {
                if num_vec[i + 1] - num_vec[i] > 3 {
                    return false;
                }else if num_vec[i+1] -num_vec[i] < 1{
                    return false
                }
            }
        }
        _ => return false,
    }
    println!("{:?}", num_vec);
    true
}
fn validity(mut num_vec: Vec<Vec<i32>>) -> () {
    let sort_order: Vec<i8> = check_order(&mut num_vec);
    let mut valid_vec: Vec<bool> = Vec::new();
    for i in 0..sort_order.len() {
        if sort_order[i] < 3_i8 {
            valid_vec.push(check_distance(&num_vec[i], sort_order[i]));
        } else {
            valid_vec.push(false);
        }
    }
    let sum: i32 = valid_vec.iter().filter(|&x| *x == true).count() as i32;
    println!("{}", sum);
}
