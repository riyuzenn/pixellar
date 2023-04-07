/*
 * Copyright (c) 2023 riyuzenn <riyuzenn@gmail.com>
 * See the license file for more info
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/
use colored::Colorize;
use strip_ansi_escapes;

fn strlen(s: &str) -> usize {
    strip_ansi_escapes::strip(&s.as_bytes()).unwrap().len()
} 

pub fn say(text: &str) {
    let len = strlen(text);
 
    let mut forward = String::from("");
    forward.push_str(&String::from("-").repeat((len/2) - 1));
    forward.push_str(">");
    forward.push_str(&String::from("-").repeat(len/2));
    
    let mut backward = String::from("");
    backward.push_str(&String::from("-").repeat((len/2) - 1));
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

