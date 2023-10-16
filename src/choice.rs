use inquire;

pub fn select<'a>(prompt: &str, choices: &Vec<&'a str>) -> &'a str {
    match inquire::Select::new(prompt, choices.clone()).prompt() {
        Ok(choice) => choice,
        Err(_) => {
            print!("Please try again. ");
            select(prompt, choices)
        }
    }
}
