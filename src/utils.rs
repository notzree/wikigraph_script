//todo: move sanitize_string here
pub fn sanitize_string(input: &str) -> String {
    // Trim leading and trailing whitespace
    let trimmed = input.trim();

    // Replace all underscores with spaces
    let underscores_to_spaces = trimmed.replace('_', " ");

    // Convert to lowercase
    let lowercase = underscores_to_spaces.to_lowercase();

    // Remove colon if the first character that is not a whitespace is a colon

    if lowercase.starts_with(':') {
        lowercase.strip_prefix(':').unwrap().to_string()
    } else {
        lowercase
    }
}
