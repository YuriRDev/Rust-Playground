use highlight_search::*;

fn main() {
    println!("{} {}", TextStyle::to_string("[cyan]", TextStyle::CYAN), "<< Yeyy!");
    println!("{} {}", TextStyle::to_string("[BOLD]", TextStyle::BOLD), "<< Bold...");

}
