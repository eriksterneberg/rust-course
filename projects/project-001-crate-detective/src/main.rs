// webprobe - A CLI tool to fetch and parse JSON from URLs
//
// YOUR TASK: Implement this tool using the dependencies you chose.
//
// Requirements:
// 1. Accept a URL as a command-line argument
// 2. Make an HTTP GET request to that URL
// 3. Parse the JSON response
// 4. Print at least one field from the response
// 5. Handle errors gracefully (no panics!)
//
// Test endpoint: https://httpbin.org/json
// This returns a predictable JSON structure you can parse.

fn main() {
    // TODO: Parse command-line arguments
    // If no URL provided, print usage and exit gracefully

    // TODO: Make HTTP GET request
    // Handle connection errors, timeouts, non-200 responses

    // TODO: Parse JSON response
    // Handle malformed JSON gracefully

    // TODO: Print formatted output
    // Extract and display relevant fields

    println!("webprobe: not yet implemented");
    println!("Usage: webprobe <URL>");
    println!();
    println!("Example: webprobe https://httpbin.org/json");
}

// Hint: You might want to create a struct to deserialize into
// if using serde. For example:
//
// #[derive(Debug, serde::Deserialize)]
// struct HttpBinResponse {
//     slideshow: Slideshow,
// }
//
// #[derive(Debug, serde::Deserialize)]
// struct Slideshow {
//     title: String,
//     // ... other fields
// }
