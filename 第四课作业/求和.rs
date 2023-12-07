fn sum_u32(numbers: &[u32]) -> Option<u32> {
    let mut sum = 0u32;
    for &num in numbers {
        match sum.checked_add(num) {
            Some(s) => sum = s,
            None => return None,
        }
    }
    Some(sum)
}

fn main() {
    let numbers: [u32; 5] = [10, 20, 30, 40, 50];
    match sum_u32(&numbers) {
        Some(sum) => println!("The sum is: {}", sum),
        None => println!("Overflow occurred"),
    }
}
