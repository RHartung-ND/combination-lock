use std::io::{self, Write};

fn naive_avg (num_arr: &Vec<char>, num_digits: usize) -> String {

    let mut avg: i32 = 0;
    for i in 0..num_digits {
        // "as i32" converts to the corresponding unicode value, so you must subtract by 48 (0x30) to get actual number
        avg += num_arr[i] as i32 - 0x30;
    }

    avg = avg/num_digits as i32;
    let result: String = avg.to_string().repeat(num_digits);

    return result; 

}

fn distance (num_arr: &Vec<char>, num_digits: usize) -> String {
    let mut result = String::new();
    for i in 0..num_digits {
        // "as i32" converts to the corresponding unicode value, so you must subtract by 48 (0x30) to get actual number
        let curr_digit = num_arr[i] as i32 - 0x30;
        let mut average_distance: i32 = 0;
        for j in 0..num_digits {
            if i == j {
                continue;
            }

            let other_digit: i32 = num_arr[j] as i32 - 0x30;

            let distance1: i32 = other_digit - curr_digit;
            let distance2: i32 = (other_digit + 10) - curr_digit;
            let distance3: i32 = other_digit - (curr_digit + 10);

            // println!("{distance1} | {distance2} | {distance3}");

            if distance1.abs() <= 5 {
                average_distance += distance1;
            } else if distance2.abs() <= 5 {
                average_distance += distance2;
            } else if distance3.abs() <= 5 {
                average_distance += distance3;
            }
        }
            average_distance = average_distance/(num_digits as i32 - 1);
            println!("{average_distance}");
            let mut res_digit = average_distance + curr_digit;
            if res_digit < 0 {
                res_digit += 10;
            } else if res_digit > 9 {
                res_digit -= 10;
            }

            result.push_str(&(res_digit).to_string());
    }

    println!("result {result}");

    let num_arr: Vec<_> = result.chars().collect();
    let avg = naive_avg(&num_arr, num_digits);

    return avg;
}

fn main() {
    println!("Input your combination:");
    print!("==> ");
    io::stdout().flush().unwrap();

    let mut combo = String::new();

    io::stdin().read_line(&mut combo).unwrap();

    let combo = combo.trim();
    let num_digits = combo.chars().count();
    let num_arr: Vec<_> = combo.chars().collect();

    for i in 0..num_digits {
        if (num_arr[i] as i32) < 48 || (num_arr[i] as i32) > 57 {
            println!("Your combination must be an integer between 0 and 9.");
            return;
        }
    }

    if num_digits == 1 {
        println!("Your combination is \"{}\"", combo);
        return;
    }

    let avg = naive_avg(&num_arr, num_digits);
    println!("Your combination using naive average is \"{}\"", avg);

    let dist = distance(&num_arr, num_digits);
    println!("Your combination using distance average is \"{}\"", dist);
}
