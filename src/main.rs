// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is  9009 = 91 x 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

fn main() {
    let mut num1: u32 = 0;
    let mut num2: u32 = 0;

    let mut palindromic_list: Vec<u32> = Vec::new();

    while num1 <= 999 && num2 <= 999 {
      let result: u32 = num1 * num2;

      // check if palindromic
      if flip_number(result) == result {
        palindromic_list.push(result);
        println!("{:?} is a palindromic number", result);
      }

      // update nums
      if num1 > num2 {
        num2 += 1;
      }
      else {
        num1 += 1;
      }
      
    }
}

fn flip_number(n: u32) -> u32 {
  let str: String = format!("{:?}", n);
  let reversed = str.chars().rev().collect::<String>();
  // Convert back to u32
  let result: u32 = reversed.parse().unwrap();

  // println!("{:?}", reversed);
  return result;
}