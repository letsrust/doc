fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let data1 = data;        // data is moved to data1
    println!("{}", sum(data1));
    println!("{:?}", data1);
    // println!("{}", sum(data));
}

fn sum(data: Vec<i32>) -> i32 {
    data.iter().fold(0, |acc, x| acc + x)
}