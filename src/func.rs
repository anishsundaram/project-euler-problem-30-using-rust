pub fn split_digits(mut num: u32) -> Vec<u32> {
    //mutable clone of input
    let mut digits = Vec::new(); //Mutable vector of digits
    while num > 0 {
        //Loop to seperate by digit and push to vector
        digits.push(num % 10);
        num /= 10;
    }
    digits //return vector of digits
} //split_digits()

pub fn run() {
    let mut total_sum = 0; //Declare sum value to be returned
    let mut temp = 10; //Begin increment at 10

    while temp < 1000000 {
        //Go until a large number
        let values = split_digits(temp); //vector of each individual digit
        let mut temp_total = 0; //Total to check if sum equals original num
        for token in values {
            //loop through vector of digits
            temp_total += token.pow(5); //fifth power sum
        }
        if temp_total == temp {
            //check if sum equals original num
            total_sum += temp_total;
        }
        temp += 1; //incrementation
    }
    println!(
        "Sum of numbers that equal sum of fifth powers of their digits: {}",
        total_sum
    );
} //run()
