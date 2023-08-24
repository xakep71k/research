fn main() {
    let filename = "/home/alek/tmp/1.txt";
    let now = std::time::Instant::now();
    let lines = lines_from_file(filename);
    println!("{:?} reading input data", now.elapsed());

    custom_sort(lines.clone());
    std_sort(lines.clone());
    pdqsort(lines.clone());
    afsort(lines.clone());
    rdx(lines.clone());
}

fn custom_sort(lines: Vec<String>) {
    let mut v: Vec<&str> = lines.iter().map(|f| f.as_str()).collect();

    let now = std::time::Instant::now();
    quick_sort(&mut v);
    println!("=== {:?} custom sorting vector", now.elapsed());
    // write(v, "/home/alek/tmp/_1.txt");
}

fn std_sort(lines: Vec<String>) {
    let mut v: Vec<&str> = lines.iter().map(|f| f.as_str()).collect();

    let now = std::time::Instant::now();
    v.sort();
    println!("=== {:?} std::slice sorting vector", now.elapsed());
    // write(v, "/home/alek/tmp/_2.txt");
}

fn pdqsort(lines: Vec<String>) {
    let mut v: Vec<&str> = lines.iter().map(|f| f.as_str()).collect();

    let now = std::time::Instant::now();
    pdqsort::sort(&mut v);
    println!("=== {:?} pdqsort sorting vector", now.elapsed());
    // write(v, "/home/alek/tmp/_3.txt");
}

fn afsort(lines: Vec<String>) {
    let mut v: Vec<&str> = lines.iter().map(|f| f.as_str()).collect();

    let now = std::time::Instant::now();
    afsort::sort_unstable(&mut v);
    println!("=== {:?} afsort sorting vector", now.elapsed());
    // write(v, "/home/alek/tmp/_4.txt");
}

fn rdx(lines: Vec<String>) {
    let mut v: Vec<&str> = lines.iter().map(|f| f.as_str()).collect();

    let now = std::time::Instant::now();
    rdx::ska_sort::ska_sort(&mut v);
    println!("=== {:?} ska_sort sorting vector", now.elapsed());
    // write(v, "/home/alek/tmp/_4.txt");
}

fn lines_from_file(filename: impl AsRef<std::path::Path>) -> Vec<String> {
    use std::io::BufRead;

    let file = std::fs::File::open(filename).expect("no such file");
    let buf = std::io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn write(v: Vec<&str>, p: &str) {
    use std::io::Write;

    let mut fd = std::fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(p)
        .unwrap();
    for i in v.iter() {
        writeln!(fd, "{}", i).unwrap();
    }
}

pub fn quick_sort(arr: &mut [&str]) {
    let len = arr.len();
    _quick_sort(arr, 0, (len - 1) as isize);
}

fn _quick_sort(arr: &mut [&str], low: isize, high: isize) {
    if low < high {
        let p = partition(arr, low, high);
        _quick_sort(arr, low, p - 1);
        _quick_sort(arr, p + 1, high);
    }
}

fn partition(arr: &mut [&str], low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut store_index = low - 1;
    let mut last_index = high;

    loop {
        store_index += 1;
        while arr[store_index as usize] < arr[pivot] {
            store_index += 1;
        }
        last_index -= 1;
        while last_index >= 0 && arr[last_index as usize] > arr[pivot] {
            last_index -= 1;
        }
        if store_index >= last_index {
            break;
        } else {
            arr.swap(store_index as usize, last_index as usize);
        }
    }
    arr.swap(store_index as usize, pivot);
    store_index
}
