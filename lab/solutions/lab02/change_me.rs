fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    round_to_next_even(&mut numbers[..]);
    println!("{:#?}", numbers);
}

fn round_to_next_even(nums: &mut [u32]) {
    for num in nums.iter_mut() {
        if *num % 2 != 0 {
            *num = *num + 1
        }
    }
}
