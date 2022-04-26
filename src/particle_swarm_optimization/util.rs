
pub fn print_generic_vector<T: std::fmt::Display>(vector: &Vec<T>) -> String {
    let mut text = String::from("");
    text += "[";
    for (index, value) in vector.iter().enumerate() {
        text += &value.to_string();
        if index != vector.len() - 1 {
            text += ", ";
        }
    }
    text += "]";
    return text;
}