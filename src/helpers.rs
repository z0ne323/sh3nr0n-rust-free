use reqwest::blocking::Response;
use reqwest::Error as ReqwestError;

pub fn handle_error_shodan(result: Result<Response, ReqwestError>) {
    /*
    Description:
        Handles errors specific to Shodan module, providing a centralized error handling mechanism.
    Parameters:
        result (Result<Response, ReqwestError>): The result of a Shodan module function call, containing either a successful response or an error.
    Returns:
        None: The function does not return a value. It prints relevant information about the response or error.
    */
    match result {
        Ok(response) => {
            // Handle successful response
            println!("Status: {}", response.status());
            let body = response.text().expect("Failed to read response body");
            println!("Body:\n{}", body);
        }
        Err(err) => {
            // Handle generic error
            eprintln!("Error: {:?}", err);
        }
    }
}