/*
Given two strings needle and haystack, return the index of the first occurrence of needle in haystack,
 or -1 if needle is not part of haystack.

Example 1:

Input: haystack = "sadbutsad", needle = "sad"
Output: 0
Explanation: "sad" occurs at index 0 and 6.
The first occurrence is at index 0, so we return 0.
Example 2:

Input: haystack = "leetcode", needle = "leeto"
Output: -1
Explanation: "leeto" did not occur in "leetcode", so we return -1.


Constraints:

1 <= haystack.length, needle.length <= 104
haystack and needle consist of only lowercase English characters.
*/
fn main() {
    let s1 = String::from("sadbutsab");
    println!("Index of 'sabu' in {s1} is {}", str_str(s1.clone(), String::from("sab")));
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    match haystack.find(&needle){
        Some(index) => index as i32,
        None => -1,
    }
}