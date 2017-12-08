fn main() {
    // Read file into a string
    let contents = include_str!("../data.txt");

    //Convert string into array of digits
    let num_array:Vec<u32> = contents.chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect();

    println!("{}", sum_next_array(num_array));
}

fn sum_next_array(num_array: Vec<u32>) -> u32{

    //Check for consequent digits
    //Note : last returns an address
    let mut last = *num_array.last().unwrap();
    let mut sum = 0;
    for i in num_array {
        if i == last {
            sum += i;
        }
        last = i;
    }
    sum
}


#[cfg(test)]
mod tests {
    use sum_next_array;

    #[test]
    fn test_1() {
        let num_vec = vec![1, 1, 2, 2];
        assert_eq!(sum_next_array(num_vec), 3);
    }

    #[test]
    fn test_2() {
        let num_vec = vec![1, 1, 1, 1];
        assert_eq!(sum_next_array(num_vec), 4);
    }

    #[test]
    fn test_3() {
        let num_vec = vec![1, 2, 3, 4];
        assert_eq!(sum_next_array(num_vec), 0);
    }

    #[test]
    fn test_4() {
        let num_vec = vec![9,1,2,1,2,1,2,9];
        assert_eq!(sum_next_array(num_vec), 9);
    }
}