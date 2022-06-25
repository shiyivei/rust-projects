use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));

    opt.map_or(Ok(None), |r| r.map(Some))
}

fn main() {
    let nums = vec!["32", "46", "78"];
    let empty = vec![];
    let string = vec!["todo", "32", "78"];

    println!("the first vec is {:?}", double_first(nums));
    println!("the first vec is {:?}", double_first(empty));
    println!("the first vec is {:?}", double_first(string))
}
