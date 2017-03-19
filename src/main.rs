/*
                World Clock Server
        Written by Regan Koopmans, 15043143
               University of Pretoria
                    March 2017

*/

// For network IO

use std::io::{Read, Write, BufReader, BufRead};
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

// For file io

use std::error::Error;
use std::fs::File;
use std::path::Path;

// For time handling [external library]

extern crate chrono;
use chrono::prelude::*;
use chrono::Duration;

// For environment/argument variabls

use std::env;

// In the main function I initialize a TcpListener listener, that accepts
// new connections, and spawns a thread to handle an incoming connection
// stream.

fn main() {
    let mut domain = String::new();
    domain.push_str("0.0.0.0:");
    domain.push_str(env::args().nth(1).unwrap().as_str());
    let listener = TcpListener::bind(domain.as_str()).unwrap();
    println!("Server listening on {}...", domain);
    println!("Press Ctrl-C to exit.");
    for stream in listener.incoming() {
        thread::spawn(|| {
            let stream = stream.expect("Could initialize stream!");
            read_request(stream);
        });
    }
}

// Function that will create a string from a given filename.
// optionally, the function caller can opt to have HTTP headers
// prepended to this string, using the *add_headers* flag.

fn get_file_string(file_name : &str, add_headers: bool) -> String {
    let path = Path::new(&file_name);
    let mut file = match File::open(&path) {
        Err(error) => panic!("Could not open {}, {}",
                                file_name,error.description()),
        Ok(file) => file,
    };
    let mut file_string = String::new();
    file.read_to_string(&mut file_string)
                                    .expect("Could not read file to string.");
    let mut return_string = String::new();

    // HTTP headers for successful, note \r\n is required for both Unix and 
    // windows support.

    if add_headers {
        return_string.push_str("HTTP/1.1 200 OK\r\n");
        return_string.push_str("Content-Length: ");
        return_string.push_str(&(file_string.len()).to_string());
        return_string.push_str("\r\n");
        let mut content_type = "Content-Type: text/plain";
        if file_name.contains(".html") {
            content_type = "Content-Type: text/html\r\n";
        } else if file_name.contains(".css") {
            content_type = "Content-Type: text/css\r\n";
        } else if file_name.contains(".css") {
            content_type = "Content-Type: text/javascript\r\n";
        }
        return_string.push_str(content_type);
        return_string.push_str("Connection: close\r\n\r\n");
    }
    return_string.push_str(&file_string);
    return_string
}

// Function that reads and interprets an HTTP 1.1 request.
// This function also maps abstract paths (like main.css) to
// absolute paths in the file system of the server.

fn read_request(stream: TcpStream) {
    let mut response = ("", true);
    let mut reader = BufReader::new(stream);
    // println!("\n\n\x1B[1;32mNEW TCP STREAM\x1B[0m\n");
    for line in reader.by_ref().lines() {
        let line = line.unwrap();
        if line == "" {
            break;
        } else {
            let line_array: Vec<&str> = line.split(" ").collect();

            // If the first part of the line is 'GET'
            if line_array[0] == "GET" {

                // highlight to show the get requests.
                println!("\x1B[1;33m{}\x1B[0m", line);

                // respnse contains the tuple of the form (content, is_file?)
                response = match line_array[1] {
                    
                    // Static files data

                    "/"             => ("static/html/main.html",true),
                    "/main.css"     => ("static/css/main.css",  true),
                    "/main.js"      => ("static/js/main.js",    true),
                    "/favicon.ico"  => ("static/html/404.html", true),
                    
                    // Cities data

                    "/za"           => ("za",                   false),
                    "/xml/za"       => ("x-za",                 false),

                    "/ny"           => ("ny",                   false),
                    "/xml/ny"       => ("x-ny",                 false),

                    "/paris"        => ("paris",                false),
                    "/xml/paris"    => ("x-paris",              false),

                    "/adel"         => ("adel",                 false),
                    "/xml/adel"     => ("x-adel",               false),
                    
                    "/sao"          => ("sao",                  false),
                    "/xml/sao"      => ("x-sao",                false),
                    
                    "/beij"         => ("beij",                 false),
                    "/xml/beij"     => ("x-beij",               false),

                    "/ndel"         => ("ndel",                 false),
                    "/xml/ndel"     => ("x-ndel",               false),
                    
                    "/dub"          => ("dub",                  false),
                    "/xml/dub"      => ("x-dub",                false),
                    
                    "/mosc"         => ("mosc",                 false),
                    "/xml/mosc"     => ("x-mosc",               false),
                    
                    "/tok"          => ("tok",                  false),
                    "/xml/tok"      => ("x-tok",                false),
                    

                    "/mars"         => ("mars",                 false),
                    "/xml/mars"     => ("x-mars",               false),

                    _               => ("static/html/404.html", true),
                }
            }
        }
    }
    match response.1 {
        true  => write_response(reader.into_inner(),response.0, true),
        false => write_response(reader.into_inner(),response.0, false),
    }
}

// Function that writes the HTTP response in binary format to the 
// TCPStream connected to the user agent.

fn write_response(mut stream: TcpStream, input:&str, is_file: bool) {
    if is_file {
        stream.write_all(get_file_string(input, true).as_bytes()).unwrap();
    } else {
        stream.write(get_template(input).as_bytes()).unwrap();
    }
    stream.flush().expect("Could not flush stream!");
}

// Function that will generate an HTML page based on a country input, "za"
// for example. This function automatically prepends HTTP headers.

fn get_template(input: &str) -> String {
    let mut is_xml = false;
    let mut input = input.to_string();
    if input.contains("x-") {
        input = input.replace("x-","");
        is_xml = true;
    }
    let date_format = "%H:%M:%S";
    let result = match input.as_str() {
        "za"    => (UTC::now() + Duration::hours(2)).format(date_format).to_string(),
        "ny"    => (UTC::now() - Duration::hours(5)).format(date_format).to_string(),
        "paris" => (UTC::now() + Duration::hours(1)).format(date_format).to_string(),
        "adel"  => (UTC::now() + Duration::hours(9) + Duration::minutes(30)).format(date_format).to_string(),
        "sao"   => (UTC::now() - Duration::hours(1)).format(date_format).to_string(),
        "beij"  => (UTC::now() + Duration::hours(8)).format(date_format).to_string(),
        "ndel"  => (UTC::now() + Duration::hours(5) + Duration::minutes(30)).format(date_format).to_string(),
        "dub"   => (UTC::now() + Duration::hours(4)).format(date_format).to_string(),
        "mosc"  => (UTC::now() + Duration::hours(3)).format(date_format).to_string(),
        "tok"   => (UTC::now() + Duration::hours(9)).format(date_format).to_string(),
        "mars"  => (UTC::now() - Duration::hours(2) + Duration::minutes(1.02777777778)).format(date_format).to_string(),
        _ => "not implemented".to_string()
    };

    // get title

    let title = match input.as_str() {
        "za"    => "South Africa",
        "ny"    => "New York",
        "paris" => "Paris",
        "adel"  => "Adelaide",
        "sao"   => "São Paulo",
        "beij"  => "北京 (Beijing)",
        "ndel"  => "नई दिल्ली (New Delhi)",
        "dub"   => "دبي (Dubai)",
        "mosc"  => "Москва (Moscow)",
        "tok"   => "東京 (Tokyo)",
        "mars"   => "Mars (MTC)",
        _       => "somewhere over the rainbox"
    };

    // get the template file, add headers

    let mut template;
    let mut return_string = String::new();
    return_string.push_str("HTTP/1.1 200 OK\r\n");
    return_string.push_str("Content-Length: ");
    if !is_xml {
        template = get_file_string("static/html/template.html", false);
        template = template.replace("{{title}}", &title);
        template = template.replace("{{country}}", &input);
        template = template.replace("{{time}}", &result);
    } else {
        template = result;
    }
    return_string.push_str(&(template.len()).to_string());
    return_string.push_str("\r\n");
    return_string.push_str("Content-Type: text/html\r\n");
    return_string.push_str("Connection: close\r\n\r\n");
    return_string.push_str(&template);
    return_string
}
