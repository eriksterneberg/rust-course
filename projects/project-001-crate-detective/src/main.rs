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

use clap::Parser;
use reqwest::blocking;

fn main() {
    // TODO: Parse command-line arguments
    // If no URL provided, print usage and exit gracefully
    let args = Args::parse();                                                                             

    println!("Sending request with url {}", args.url);

    // Handle connection errors, timeouts
    let response = match blocking::get(args.url) {
        Ok(resp) => resp,
        Err(e) => {
            if e.is_timeout() {
                eprint!("Error: Request timed out");
            } else if e.is_connect() {
                eprint!("Error: Unable to connect");
            } else {
                eprintln!("Error: {}", e);
            }
            std::process::exit(1);
        }
    };

    // Handle non-200 responses
    if !response.status().is_success() {
        eprint!("Error: status was {}", response.status());
        std::process::exit(1);
    }

    // TODO: Parse JSON response
    // Handle malformed JSON gracefully
    let json: serde_json::Value = match response.json() {
        Ok(result) => result,
        Err(e) => {
                eprintln!("JSON Parse Error: {}", e);
            std::process::exit(1);
        }
    };

    // TODO: Print formatted output
    println!("Body = {:#?}", json);

    // Extract and display relevant fields
    if let Some(slideshow) = json.get("slideshow") {                                                                     
    if let Some(title) = slideshow.get("title") {                                                                    
        println!("{}", title);                                                                                       
    }                                                                                                                
  } 
}

  /// A simple program to greet someone                                                                     
  #[derive(Parser, Debug)]                                                                                  
  #[command(name = "Webprobe Arguments")]                                                                              
  #[command(about = "Arguments for the simple Webprobe CLI tool", long_about = None)]                                                    
  struct Args {                                                                                             
      url: String,                                                                                         
  } 
