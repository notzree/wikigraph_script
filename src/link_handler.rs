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

        let mut chars = text.chars().peekable();
        while let Some(c) = chars.next() {
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
                        if current_link.contains("(disambiguation)") {
                            inside_link = false;
                            current_link.clear();
                            continue;
                        }
                        links.push(sanitize_string(&current_link));
                        inside_link = false;
                    }
                }
                '<' => {
                    //we encounter a tag, based on the markup, we can skip the content of the tag...
                    let mut tag_name = String::new();
                    let mut closing_tag_name = String::new();
                    //TODO:check if tag_name == closing_tag_name..
                    for c in chars.by_ref() {
                        if c == '>' {
                            break;
                        }
                        tag_name.push(c);
                    }
                    if tag_name.contains("ref") || tag_name.contains("syntaxhighlight") {
                        //we skip the content of the tag
                        while let Some(c) = chars.next() {
                            if c == '<' && chars.peek() == Some(&'/') {
                                //encounter closing tag
                                for c in chars.by_ref() {
                                    if c == '>' && closing_tag_name.contains("ref")
                                        || closing_tag_name.contains("syntaxhighlight")
                                    {
                                        break;
                                    }
                                    closing_tag_name.push(c);
                                }
                                break;
                            }
                        }
                    }
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

                _ if inside_link => {
                    current_link.push(c);
                    if current_link == "File:"
                        || current_link == "Wikipedia:"
                        || current_link == "WP:"
                        || current_link == "Template:"
                        || current_link == "MOS:"
                        || current_link == "Help:"
                        || current_link == "Draft:"
                        || current_link == "User:"
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
