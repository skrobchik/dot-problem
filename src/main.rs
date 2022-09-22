use rayon::prelude::*;

fn bit_string_contains(string: u32, string_len: u32, substring: u32, substring_len: u32) -> bool {
    if string_len < substring_len {
        return false;
    }
    if substring_len == 0 {
        return true;
    }
    let bit_mask: u32 = (1 << substring_len) - 1;
    for shift in 0..string_len - substring_len + 1 {
        let shifted_bit_mask = bit_mask << shift;
        let masked_string = string & shifted_bit_mask;
        let shifted_substring = substring << shift;
        if masked_string ^ shifted_substring == 0 {
            return true;
        }
    }
    false
}

fn bit_string_contains_all<T>(
    string: u32,
    string_len: u32,
    substrings: T,
    substring_len: u32,
) -> bool
where
    T: IntoIterator<Item = u32>,
{
    substrings.into_iter().all(|substring| {
        bit_string_contains(string, string_len, substring, substring_len)
    })
}

fn bit_string_contains_all_par<T>(
    string: u32,
    string_len: u32,
    substrings: T,
    substring_len: u32,
) -> bool
where 
    T: IntoIterator<Item = u32> + IntoParallelIterator<Item = u32>,
{
    substrings.into_par_iter().all(|substring| {
        bit_string_contains(string, string_len, substring, substring_len)
    })
}

fn main() {
    let substring_len = 5;
    let substrings = 0..(1 << substring_len);
    let max_string_len = 32;
    let mut found = false;
    for string_len in 0..max_string_len {
        println!("string_len: {}", string_len);
        for string in 0 .. (1 << string_len) - 1 {
            if bit_string_contains_all_par(string, string_len, substrings.clone(), substring_len) {
                // print `string` cropped to the `string_len` least significant bits:
                println!("{:0width$b}", string, width = string_len as usize);
                found = true;
            }
        }
        if found {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(bit_string_contains(0b001, 3, 0b01, 2));
        assert!(bit_string_contains(0b101, 3, 0b01, 2));
        assert!(bit_string_contains(0b1010, 4, 0b10, 2));
        assert!(bit_string_contains(0b1010, 4, 0b01, 2));
        assert!(bit_string_contains(0b101, 3, 0b10, 2));
        assert!(bit_string_contains(0b101, 3, 0b01, 2));
        assert!(!bit_string_contains(0b1010, 4, 0b11, 2));        
        assert!(bit_string_contains_all(0b1010, 4, vec![0b10, 0b01], 2));
        assert!(!bit_string_contains_all(0b1010, 4, vec![0b10, 0b11], 2));
    }

    #[test]
    fn test2() {
        assert!(bit_string_contains(0b01010101, 8, 0, 0));
    }
}