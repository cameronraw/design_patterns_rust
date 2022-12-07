use std::ops::Add;

pub fn create_title(title: &str){
    let title_length = title.chars().count();
    let mut title_border = String::new();
    for _ in 0..title_length {
        title_border = title_border.add("-");
    }

    println!("\n{}", title_border);
    println!("{}", title);
    println!("{}\n", title_border);
}
