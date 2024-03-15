use std::thread::current;

//move link extraction here
use crate::utils::sanitize_string;
pub trait LinkHandler {
    fn extract_links(&self, input: String) -> Vec<String>;
}

pub struct WikiLinkHandler;
impl LinkHandler for WikiLinkHandler {
    fn extract_links(&self, text: String) -> Vec<String> {
        let mut links: Vec<String> = Vec::new();
        let mut current_link = String::new();
        let mut inside_link = false;
        let mut inside_tag = false; // New variable to track if we're inside a tag
        let mut tag_depth = 0; // Track the depth of nested tags

        let mut chars = text.chars().peekable();
        while let Some(c) = chars.next() {
            if inside_tag {
                // Logic to handle skipping tags, including nested ones
                if c == '<' {
                    tag_depth += 1; // Increase depth for nested tags
                } else if c == '>' {
                    tag_depth -= 1; // Decrease depth when closing tags
                    if tag_depth == 0 {
                        inside_tag = false; // We're no longer inside a tag
                    }
                    continue; // Skip the rest of the loop iteration
                }
                // Skip characters inside tags
                continue;
            }
            match c {
                '[' if chars.peek() == Some(&'[') => {
                    // Detect starting "[["
                    chars.next(); // Skip the next '['
                    inside_link = true;
                    current_link.clear();
                }
                ']' if chars.peek() == Some(&']') => {
                    //end links
                    // Detect ending "]]"
                    chars.next(); // Skip the next ']' as it's part of the marker
                    if inside_link && !current_link.is_empty() {
                        if current_link.contains('|') {
                            let mut split = current_link.split('|');
                            let link = split.next().unwrap();
                            current_link = link.to_string();
                        }
                        if current_link.contains('#') {
                            //Some redirets havee a #and a specific A-tag, we ignore them.
                            let mut split = current_link.split('#');
                            let link = split.next().unwrap();
                            current_link = link.to_string();
                        }
                        if current_link.contains("(disambiguation)")
                            || sanitize_string(&current_link).is_empty()
                        //make sure the sanitized version is valid
                        {
                            inside_link = false;
                            current_link.clear();
                            continue;
                        } else {
                            links.push(sanitize_string(&current_link));
                            inside_link = false;
                        }
                    }
                }
                '<' => {
                    //skip all tags
                    inside_tag = true;
                    tag_depth = 1;
                }
                '{' if chars.peek() == Some(&'{') => {
                    //skip till the end
                    while let Some(c) = chars.next() {
                        if c == '}' && chars.peek() == Some(&'}') {
                            chars.next();
                            break;
                        }
                    }
                }

                _ if inside_link && !inside_tag => {
                    current_link.push(c);
                    if current_link == "File:"
                        || current_link == "Wikipedia:"
                        || current_link == "WP:"
                        || current_link == "Template:"
                        || current_link == "MOS:"
                        || current_link == "Help:"
                        || current_link == "Draft:"
                        || current_link == "User:"
                        || current_link == "Image:"
                    {
                        // we realize that we are in either a file, template, or wikipedia article namespace. We reseet
                        inside_link = false;
                        current_link.clear();
                    }
                }
                _ => {}
            }
        }
        links
    }
}
