pub enum Align {
    Left,
    Right,
}

fn get_indent(print_text: &str, indention_text: &str, indention: usize, align: Align) -> String {
    match align {
        Align::Left => {
            if indention < indention_text.len() {
                return "".to_string();
            }
            ' '.to_string().repeat(indention - indention_text.len())
        }
        Align::Right => {
            if indention < (indention_text.len() + print_text.len()) {
                return "".to_string();
            }
            ' '.to_string()
                .repeat(indention - indention_text.len() - print_text.len() + 1)
        }
    }
}

pub fn print(print_text: &str, indention_text: &str, indention: usize, align: Align) {
    let indent = get_indent(print_text, indention_text, indention, align);
    print!("{}{}", indent, print_text);
}

pub fn println(print_text: &str, indention_text: &str, indention: usize, align: Align) {
    let indent = get_indent(print_text, indention_text, indention, align);
    println!("{}{}", indent, print_text);
}
