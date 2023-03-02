fn main() {
    let list1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let list2 = vec![6.0, 7.0, 8.0, 9.0, 10.0];
    let window_size = 3;

    let moving_avg1 = moving_average(&list1, window_size);
    let moving_avg2 = moving_average(&list2, window_size);

    println!("List 1: {:?}", list1);
    println!("Moving average of List 1: {:?}", moving_avg1);
    println!("List 2: {:?}", list2);
    println!("Moving average of List 2: {:?}", moving_avg2);
}

fn moving_average(list: &Vec<f64>, window_size: usize) -> Vec<f64> {
    let mut moving_avg = Vec::new();

    for i in window_size..list.len() {
        let sum: f64 = list[i - window_size..i].iter().sum();
        moving_avg.push(sum / window_size as f64);
    }

    moving_avg
}
