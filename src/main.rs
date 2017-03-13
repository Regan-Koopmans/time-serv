// For network IO

use std::io::{Read, Write, BufReader, BufRead};
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

// For file io

use std::error::Error;
use std::fs::File;
use std::path::Path;

// For time handling

extern crate chrono;
use chrono::prelude::*;

// For env variabls

use std::env;

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

fn get_file_string(file_name : &str, add_headers: bool) -> String {
    let path = Path::new(&file_name);
    let mut file = match File::open(&path) {
        Err(error) => panic!("Could not open {}, {}",
                                file_name,error.description()),
        Ok(file) => file,
    };
    let mut file_string = String::new();
    file.read_to_string(&mut file_string).expect("Could not read file to string.");
    let mut return_string = String::new();

    // HTTP headers for successful
    if add_headers {
        return_string.push_str("HTTP/1.1 200 OK\n");
        return_string.push_str("Content-Length: ");
        return_string.push_str(&(file_string.len()).to_string());
        return_string.push_str("\n");
        let mut content_type = "Content-Type: text/plain";
        if file_name.contains(".html") {
            content_type = "Content-Type: text/html\n";
        } else if file_name.contains(".css") {
            content_type = "Content-Type: text/css\n";
        } else if file_name.contains(".css") {
            content_type = "Content-Type: text/javascript\n";
        }
        return_string.push_str(content_type);
        return_string.push_str("Connection: close\n\n");
    }
    return_string.push_str(&file_string);
    return_string
}

//  This is the function where I pass through the entire
//  request string, and find the details I need

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

                // respnse contains the tuple of the form (content, file?)

                response = match line_array[1] {
                    "/"             => ("static/html/main.html",    true),
                    "/main.css"     => ("static/css/main.css",      true),
                    "/main.js"      => ("static/js/main.js",        true),
                    "/za"           => ("za",                       false),
                    "/xml/za"       => ("x-za",                     false),
                    "/favicon.ico"  => ("static/html/404.html",     true),
                    _               => ("static/html/404.html",     true),
                }
            } else {
                // println!("{}", line);
            }
        }
    }
    match response.1 {
        true  => write_response(reader.into_inner(),response.0, true),
        false => write_response(reader.into_inner(),response.0, false),
    }
}

fn write_response(mut stream: TcpStream, input:&str, is_file: bool) {
    if is_file {
        stream.write_all(get_file_string(input, true).as_bytes()).unwrap();
    } else {
        stream.write(get_template(input).as_bytes()).unwrap();
    }
    stream.flush().expect("Could not flush stream!");
}

fn get_template(input: &str) -> String {
    let mut is_xml = false;
    let mut input = input.to_string();
    if input.contains("x-") {
        input = input.replace("x-","");
        is_xml = true;
    }
    let date_format = "%H:%M:%S";
    let result = match input.as_str() {
        "za" => Local::now().format(date_format).to_string(),
        _ => "not implemented".to_string()
    };

    // get the template file

    let mut template;
    if !is_xml {
        template = get_file_string("static/html/template.html", false);
        template = template.replace("{{country}}", &input);
        template = template.replace("{{time}}", &result);
    } else {
        template = result;
    }
    let mut return_string = String::new();
    return_string.push_str(&template);
    return_string
}
