fn main() {
    // Read file into a string
    let contents = include_str!("../data.txt");

    //Convert string into array of digits
    let num_array:Vec<u32> = contents.chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect();



    println!("{}", sum_mid_array(num_array));
}

fn sum_mid_array(num_array : Vec<u32>) -> u32{
    
    //Watch the call order, skip last or cycle over splitted list
    let mut fwd5 = num_array.iter().cycle().skip(num_array.len()/2);
    //Check for consequent digits
    //Note : last returns an address
    let mut sum = 0;
    for i in &num_array {
        if i == fwd5.next().unwrap() {
            sum += i;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use sum_mid_array;

    #[test]
    fn test_1() {
        let num_vec = vec![1, 2, 1, 2];
        assert_eq!(sum_mid_array(num_vec), 6);
    }

    #[test]
    fn test_2() {
        let num_vec = vec![1, 2, 2, 1];
        assert_eq!(sum_mid_array(num_vec), 0);
    }

    #[test]
    fn test_3() {
        let num_vec = vec![1, 2, 3, 4, 2 ,5];
        assert_eq!(sum_mid_array(num_vec), 4);
    }

    #[test]
    fn test_4() {
        let num_vec = vec![1,2,3,1,2,3];
        assert_eq!(sum_mid_array(num_vec), 12);
    }

    #[test]
    fn test_5() {
        let num_vec = vec![1,2,1,3,1,4,1,5];
        assert_eq!(sum_mid_array(num_vec), 4);
    }
}