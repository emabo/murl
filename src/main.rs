use std::io::Write;
use std::fs::File;
use clap::{Arg, App};
use curl::easy::Easy;

fn main() {
    let matches = App::new("Rust Downloader")
        .version("0.1.0")
        .author("Emanuele Bovisio")
        .about("A Rust downloader based on libcurl")
        .arg(Arg::with_name("url")
            .short("u")
            .long("url")
            .value_name("URL")
            .help("URL of the file to download")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("FILE")
            .help("Destination path for the downloaded file")
            .takes_value(true)
            .required(true))
        .get_matches();

    // Get the URL and output path from the user
    let url = matches.value_of("url").unwrap();
    let output_path = matches.value_of("output").unwrap();

    // Create or overwrite the destination file
    let mut file = File::create(output_path).expect("Unable to create the file");

    // Initialize an Easy object to configure the request
    let mut easy = Easy::new();

    // Set the download URL
    easy.url(url).unwrap();

    // Write the downloaded data to the file
    easy.write_function(move |data| {
        file.write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();

    // Perform the download request
    easy.perform().unwrap();

    println!("Download completed!");
}
