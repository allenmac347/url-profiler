use clap::{Arg, App}; 
use openssl::ssl::{SslMethod, SslConnector};
use std::io::{Read,Write}; 
use std::net::TcpStream;
use std::collections::HashSet;
use std::time::Instant; 


fn main(){
    main_driver(); 
}

fn main_driver() -> i32{
    let matches = App::new("Cloudflare Systems Assignment")
        .version("1.0")
        .author("Allen Macaspac <amaca@umich.edu>")
        .about("Time http requests")
        .arg(Arg::new("url")
            .short('u')
            .long("url")
            .value_name("INPUT_URL")
            .about("Set the url to time")
            .takes_value(true)
            .required(true))
        .arg(Arg::new("profile")
            .short('p')
            .long("profile")
            .value_name("PROFILE")
            .about("Set number of request to make to input URL")
            .takes_value(true)
            .required(true))
        .get_matches(); 

    let mut url = "";
    if let Some(c) = matches.value_of("url"){
        url = c; 
    }

    let mut prof = 0; 
    if let Some(p) = matches.value_of("profile"){
        let parsed = p.parse::<i32>();
        match parsed {
            Ok(_val) => (),
            Err(_why) => {
                println!("\"{}\" is not a number", p); 
                return 1; 
            }, 
        }
        prof = p.parse::<i32>().unwrap(); 
    }

    let ret_value = profile_url(url, prof);
    return ret_value; 
}

fn find_median(input_times: &Vec<f32>) -> f32{
    if input_times.len() % 2 == 0{
        return (input_times[input_times.len() / 2 - 1] + input_times[(input_times.len() / 2)]) / 2.0; 
    }
    return  input_times[(input_times.len() - 1) / 2]; 
}

fn find_mode(input_times: &Vec<f32>) -> f32{
    let mut max_value = 0.0;
    let mut max_counts = 0;  
    let mut current_value = -1.0; 
    let mut current_counts = 0; 
    for time in input_times.iter(){
        if *time != current_value{
            if max_counts < current_counts{
                max_value = current_value; 
                max_counts = current_counts; 
            }
            current_value = *time; 
            current_counts = 1; 
        }
        else{
            current_counts += 1; 
        }
    }
    if max_counts < current_counts{
        max_counts = current_counts;
        max_value = current_value; 
    }
    return max_value; 
}

fn profile_url(input_url: &str, profile_value: i32) -> i32{
    println!("input url: {}", input_url); 
    println!("input prof: {}", profile_value); 
    let url_splits: Vec<&str> = input_url.splitn(2, '/').collect(); 
    let host = url_splits[0]; 
    let mut path = ""; 
    if url_splits.len() > 1 && url_splits[1].len() > 0{
        println!("{}", url_splits[1].len()); 
        path = url_splits[1]; 
    }
    let mut times = Vec::<f32>::new(); 
    let mut big_size = 0; 
    let mut small_size = std::usize::MAX; 
    let mut errors = 0; 
    let mut error_codes = HashSet::<i32>::new(); 
    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
    for _x in 0..profile_value {
        let http_addr = [host, ":443"].join(""); 
        let mut http_request = String::new(); 
        http_request.push_str(&format!("GET /{} HTTP/1.1\r\n", path).to_owned()); 
        http_request.push_str(&format!("Host: {}\r\n", host).to_owned()); 
        http_request.push_str(&format!("Accept: */* \r\n").to_owned());
        http_request.push_str(&format!("Connection: close\r\n\r\n").to_owned()); 
        println!("Sending request: \r\n{}", http_request);
        let start = Instant::now(); 
        let stream = match TcpStream::connect(http_addr){
            Ok(stream) => stream,
            Err(e) =>{
                println!("Error connecting to host: {}\r\n", e);
                errors += 1; 
                continue; 
            }
        };
        let mut stream = connector.connect(host, stream).unwrap();
        let response = stream.write_all(http_request.as_bytes());
    
        
        match response {
            Ok(v) => println!("Message recieved while sending: {:?}", v),
            Err(e) => {
                println!("Recieved error while sending: {}\r\n", e);
                errors += 1; 
                continue; 
            }
        }
        
        let mut buf = Vec::<u8>::new();
        let result = stream.read_to_end(&mut buf);
        match result {
            Ok(bytes) => {
                println!("Read {} bytes", bytes); 
                if bytes < small_size{
                    small_size = bytes; 
                }
                if bytes > big_size {
                    big_size = bytes; 
                }
            },
            Err(e) => {
                println!("Recieved error while reading: {}\r\n", e);
                errors += 1; 
                continue; 
            }
        }
        let old = Instant::now(); 
        let message = String::from_utf8_lossy(&buf);
        let parsed_message: Vec<&str> = message.splitn(2, "\r\n").collect(); 
        let response_code: Vec<&str> = parsed_message[0].split(" ").collect(); 
        let response_number = response_code[1].parse::<i32>().unwrap();
        if response_number > 299{
            error_codes.insert(response_number); 
            errors += 1; 
        }
        let total_runtime = (old.duration_since(start).as_secs_f32()*1000.0).round() / 1000.0;
        println!("Status code: \r\n{}", response_number); 
        println!("Time for response: {:?}\r\n", total_runtime); 
        times.push(total_runtime); 

    }
    if errors == profile_value{
        println!("Warning: Not able to produce any successful requests");
        println!("Please examine output log for requests made"); 
        if error_codes.len() > 0{
            println!("Error codes recieved:"); 
            for code in error_codes{
                println!("{}", code); 
            }
        }
        return 1; 
    }
    //Output general statistics
    println!("Total number of requests: {}", profile_value); 
    println!("Total errors: {}", errors); 
    println!("Percentage of requests succeeded: {}%", ((profile_value - errors) / profile_value) * 100); 
    println!("Size of biggest response: {} bytes", big_size); 
    println!("Size of smallest response: {} bytes", small_size); 
    
    //Find min, max, median, and mode of times 
    times.sort_by(|a, b| a.partial_cmp(b).unwrap()); 
    println!("Fastest time: {}s", times.first().unwrap()); 
    println!("Slowest time: {}s", times.last().unwrap()); 
    println!("Median time: {}s", (find_median(&times)*10000.0).round() / 10000.0); 
    println!("Mode time: {}s", find_mode(&times));
    if error_codes.len() > 0{
        println!("Error codes recieved:"); 
        for code in error_codes{
            println!("{}", code); 
        }
    }
    else{
        println!("Error codes recieved: none"); 
    }
    return 0; 
}