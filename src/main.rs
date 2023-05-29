fn main() {
    let mut input = "gni rts";
    let mut comparator = "str ing";
    input = input.trim();
    comparator = comparator.trim();
    println!("input: {}, comparator: {}", input, comparator);
    if input.len() != comparator.len() {
        println!("not permutration");
        std::process::exit(1);
    }

    let input_array = to_array::<7>(input);
    let comparator_array = to_array::<7>(comparator);

    for (i, _) in input_array.iter().enumerate() {
        if input_array[i] != comparator_array[input_array.len() - 1 - i] {
            println!("not permutration");
            std::process::exit(1);
        }
    }

    println!("permutration");
}

fn to_array<const N: usize>(s: &str) -> [char; N] {
    let mut chars = s.chars();
    [(); N].map(|_| chars.next().unwrap())
}
