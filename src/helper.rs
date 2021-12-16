use colored::{self, Colorize};
use std::{collections::HashMap, fmt, fs};

pub fn load_input_for_day(day: usize) -> String {
    // load input
    fs::read_to_string(format!("inputs/day{}.txt", day)).expect("input file missing!")
}

pub struct OutputFormatter {
    task: usize,
    fields: HashMap<String, Box<dyn fmt::Debug>>,
}
impl OutputFormatter {
    pub fn new(task: usize) -> OutputFormatter {
        OutputFormatter {
            task,
            fields: HashMap::new(),
        }
    }
    pub fn var<T: 'static + fmt::Debug>(&mut self, name: &str, value: T) -> &mut Self {
        self.fields.insert(name.to_string(), Box::new(value));
        self
    }
    pub fn print(&self) {
        let task = format!("(Task {} ): ", self.task.to_string().bold()).yellow();
        print!("{}", task);
        let mut field_iter = self.fields.iter();
        if self.fields.len() == 1 {
            if let Some((k, v)) = field_iter.next() {
                println!(" {}={}", k.green(), format!("{:?}", v).blue());
            }
            return
        } else {
            println!();
        }
        let longest_field = self.fields.keys().max_by_key(
                | field | field.len()
        ).unwrap().len();
        for (k, v) in field_iter {
            let padding = " ".repeat(longest_field - k.len());
            println!("\t↳{}{} ={}", k.green(), padding, format!("{:?}", v).blue());
        }
    }
}

pub fn out(task: usize) -> OutputFormatter {
    OutputFormatter::new(task)
}

mod tests {
    #[test]
    fn test() {
        use super::out;
        out(1).var("sum", 2345).var("funny", [12, 324, 523]).print()
    }
}
