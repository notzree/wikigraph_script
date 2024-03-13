pub fn sanitize_string(input: &str) -> String {
    // Trim leading and trailing whitespace
    let trimmed = input.trim();

    // Replace all underscores with spaces and new lines with spaces
    let mut text = trimmed.replace('_', " ");
    text = text.replace("\r\n", " ");
    text = text.replace('\n', " ");

    // Convert to lowercase
    let lowercase = text.to_lowercase();

    // Remove colon if the first character that is not a whitespace is a colon

    if lowercase.starts_with(':') {
        lowercase.strip_prefix(':').unwrap().to_string()
    } else {
        lowercase
    }
}
