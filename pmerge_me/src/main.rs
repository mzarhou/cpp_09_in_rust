use std::{env, time::Instant};

fn merge_insert_sort(vec: &mut [i32]) {
    if vec.len() < 2 {
        return;
    }
    if vec.len() > 5 {
        let mid = vec.len() / 2;
        merge_insert_sort(&mut vec[..mid]);
        merge_insert_sort(&mut vec[mid..]);
        let sorted_left_half = vec[..mid].to_vec();
        let sorted_right_half = vec[mid..].to_vec();
        merge(vec, &sorted_left_half, &sorted_right_half);
    } else {
        insertion_sort(vec);
    }
}

fn merge(vec: &mut [i32], l: &[i32], r: &[i32]) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < l.len() && j < r.len() {
        if l[i] <= r[j] {
            vec[k] = l[i];
            i += 1;
        } else {
            vec[k] = r[j];
            j += 1;
        }
        k += 1;
    }

    while i < l.len() {
        vec[k] = l[i];
        i += 1;
        k += 1;
    }

    while j < r.len() {
        vec[k] = r[j];
        j += 1;
        k += 1;
    }
}

fn insertion_sort<T: Ord>(vec: &mut [T]) {
    for i in 1..vec.len() {
        let mut j = i;
        while j > 0 && vec[j] < vec[j - 1] {
            vec.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut arr: Vec<i32> = args
        .iter()
        .skip(1)
        .map(|item| item.parse::<i32>().unwrap())
        .collect();

    println!("before:  {:?}", arr);

    let now = Instant::now();
    merge_insert_sort(&mut arr);
    let elapsed = now.elapsed();

    println!("after:   {:?}", arr);
    println!(
        "\n\nTime to process a range of {} elements with Vec<i32> {:?}\n\n",
        arr.len(),
        elapsed
    );
}
