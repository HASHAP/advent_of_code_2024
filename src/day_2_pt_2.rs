use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn day_2_p_2() {
    println!("day 2 part 2");
    let mut str_vec: Vec<String> = Vec::new();
    let _a = read_text(&mut str_vec, "input_2.txt");
    let mut num_vec: Vec<Vec<i32>> = create_num_vec(&mut str_vec);
    println!("{:?}", num_vec);
}
//function to read txt file and convert eah line into a String
fn read_text<P: AsRef<Path>>(
    input_vector: &mut Vec<String>,
    filename: P,
) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        input_vector.push(line?);
    }
    Ok(())
}
//function to convert lines of String into a i32 2-d vector
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
                } else if num_vec[i] - num_vec[i + 1] < 1 {
                    return false;
                }
            }
        }
        0 => {
            for i in 0..(num_vec.len() - 1) {
                if num_vec[i + 1] - num_vec[i] > 3 {
                    return false;
                } else if num_vec[i + 1] - num_vec[i] < 1 {
                    return false;
                }
            }
        }
        _ => return false,
    }
    println!("{:?}", num_vec);
    true
}
fn check_validity(num_vec: mut Vec<Vec<i32>>) ->(){

}
