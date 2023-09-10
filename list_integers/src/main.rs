use std::collections::HashMap;

fn main() {
    let list = vec![1, 2, 3, 3, 4, 5, 5, 7, 7, 7, 8, 11];
    println!("The list to calculate is {:?}", list);
    let rv = ResultValues::new(list);
    println!("the vec has average {} and median {} with most element is {}",
             rv.average, rv.median, rv.occurs_most);
}

struct ResultValues {
    average: i32,
    median: i32,
    occurs_most: i32,
}

impl ResultValues {
    fn new(vec: Vec<i32>) -> ResultValues {
        let length = vec.len();
        if length <= 0 {
            return ResultValues {
                average: 0,
                median: 0,
                occurs_most: 0,
            };
        }
        let median = match length%2 == 0 {
            true => vec[length/2],
            _ => vec[length/2+1],
        };
        let mut map = HashMap::new();
        let mut most_appearing: i32 = 0;
        let mut occurs_most: i32 = 0;
        let mut sum: i32 = 0;
        for el in vec.into_iter() {
            let occured = map.entry(el).or_insert(0);
            *occured += 1;
            if *occured > most_appearing {
                most_appearing = *occured;
                occurs_most = el;
            }
            sum += el;
        }
        ResultValues {
            average: sum / (length as i32),
            median,
            occurs_most,
        }
    }
}
