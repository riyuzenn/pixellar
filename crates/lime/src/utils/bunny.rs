use colored::Colorize;
use strip_ansi_escapes;

fn strlen(s: &str) -> usize {
    strip_ansi_escapes::strip(&s.as_bytes()).unwrap().len()
} 

pub fn say(text: &str) {
    let len = strlen(text);
 
    let mut forward = String::from("");
    forward.push_str(&String::from("-").repeat((len/2) - 2));
    forward.push_str("<");
    forward.push_str(&String::from("-").repeat(len/2));
    
    let mut backward = String::from("");
    backward.push_str(&String::from("-").repeat((len/2) - 2));
    backward.push_str("<");
    backward.push_str(&String::from("-").repeat(len/2));
    
    let mut space = String::from("");
    space.push_str(&String::from(" ").repeat(len - 2));

    println!(r#"
|{0}>|
| {1} |
| {2} |
| {1} |
|<{3}|
     \\
      \\
{{\_/}} ||
(. .)
/ > {4} 
    "#, forward, space, text.green(), backward, "♡".red());
}

