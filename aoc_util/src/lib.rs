pub fn split_and_cast(input: Vec<&str>) -> Vec<Vec<i32>> {
    input.iter().map(|x| x.split_whitespace().map(|x| x.parse().unwrap()).collect()).collect()
}