// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a hint.


// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    // char型の配列にする
    let mut c = input.chars();
    // 最初の要素で判別
    match c.next() {
        None => String::new(),
        // c.next()の値がNoneでないとき,
        Some(first) => first.to_string().to_uppercase()+c.as_str()
        // first.to_string().to_uppercase(),
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // 文字列スライスの配列を参照で受け取る
    // 配列からイテレータを作成
    // capitalize_firstは参照を受け取るのでiter()メソッドを使用する
    let mut iterable_words = words.iter();
    let mut _capitalized_words: Vec<String> = vec![];

    // イテレータを使って各要素にcapitalize_first()を適用
    // for word in iterable_words{
    //     _capitalized_words.push(capitalize_first(word).to_string())
    // }
    iterable_words
    .map(|word| capitalize_first(word))
    .for_each(|capitalizedWord| _capitalized_words.push(capitalizedWord));
    _capitalized_words

}   

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(_words: &[&str]) -> String {
    let mut _capitalized_words_vecotr: Vec<String> = vec![];
    // イテレータを使って各要素にcapitalize_first()を適用
    for word in _words.iter(){
        _capitalized_words_vecotr.push(capitalize_first(word).to_string())
    }

    // 最後に結合する
    _capitalized_words_vecotr.concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
