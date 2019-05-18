use std::collections::HashMap;

fn main() {
    let thelist = vec![1, 2, 3, 3, 4, 5, 5, 7, 7, 7, 8, 11];
    println!("the vec has average {} and median {} with most element is {}",
             average(&thelist), median(&thelist), occurs_most(&thelist));
}

fn average(vec: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for el in vec {
        sum += *el;
    }
    sum / vec.len()
}

fn median(vec: &Vec<i32>) -> i32 {
    let length = vec.len();
    let mid = if length%2 == 0 {
        length / 2
    } else {
        length / 2 + 1
    };
    vec[mid]
}

fn occurs_most(vec: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut max: i32 = 0;
    let mut res: i32 = 0;
    for el in vec {
        let mut occured = map.entry(el).or_insert(0);
        *occured += 1;
        if *occured > max {
            max = *occured;
            res = *el;
        }
    }
    res
}
