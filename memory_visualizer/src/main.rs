use std::fs;

fn main() {
    let mut input = String::new();
    println!("Enter filepath: ");

    let content_bytes = std::io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim_end();
    let content = match fs::read_to_string(trimmed) {
    Result::Ok(value) => value,
    Result::Err(error) => panic!("{}", error) 
  };
  
  println!("Your file content: \n{}", content);
  println!("Bytes read while opening file: \n{}", content_bytes);
  println!("Amount of characters in file: \n{}", count_characters_in_file(content));

}

fn count_characters_in_file(file_content: String) -> i32 {
    let mut result = 0;
    let mut last_was_whitespace = false;

    for char in file_content.chars() {
        if char.is_whitespace() {
            if last_was_whitespace == false {
                result += 1;
            }
            last_was_whitespace = true;
        } else {
            result += 1;
            last_was_whitespace = false;
        }
    }
    return result;
}