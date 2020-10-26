use clap::{Arg, App}; 
use std::sync::Arc; 
use std::io::{Read,Write}; 
use std::net::{Shutdown, TcpStream};
use std::collections::HashSet;
use std::time::Instant; 
use url::Url; 
use rustls; 
use webpki;
use webpki_roots; 


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
            .takes_value(true))
        .get_matches(); 

    let mut url = "";
    if let Some(c) = matches.value_of("url"){
        url = c; 
    }

    let mut prof = 1; 
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
    /*
    Input: Sorted array of float32 values
    Return: median of values
    */
    if input_times.len() % 2 == 0{
        return (input_times[input_times.len() / 2 - 1] + input_times[(input_times.len() / 2)]) / 2.0; 
    }
    return  input_times[(input_times.len() - 1) / 2]; 
}

fn find_mode(input_times: &Vec<f32>) -> f32{
    /*
    Input: Sorted array of float32 values
    Return: mode of values
    */
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

    //Initialize profile statistic variables
    let mut times = Vec::<f32>::new(); 
    let mut big_size = 0; 
    let mut small_size = std::usize::MAX; 
    let mut errors = 0; 
    let mut error_codes = HashSet::<i32>::new(); 


    //Parse the host and path 
    let parsed_url = Url::parse(input_url); 
    let mut host = String::new(); 
    let mut path = String::new();
    match parsed_url{
        Ok(v) =>{
            host.push_str(&String::from(v.host_str().unwrap())); 
            path.push_str(&String::from(v.path()).to_owned()); 
        }
        Err(e) =>{
            println!("Error occurred: {}", e);
            return 1;  
        }
    }
    

    //Construct the http request
    let mut http_request = String::new(); 
    http_request.push_str(&format!("GET /{} HTTP/1.1\r\n", path).to_owned()); 
    http_request.push_str(&format!("Host: {}\r\n", host).to_owned()); 
    http_request.push_str(&format!("Accept: */* \r\n").to_owned());
    http_request.push_str(&format!("Connection: close\r\n\r\n").to_owned()); 


    //Set up TLS config
    //Add trusted root web certificates and get DNS name
    let mut config = rustls::ClientConfig::new();
    config.root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
    let dns_name = webpki::DNSNameRef::try_from_ascii_str(&host).unwrap();
    let arc_config = Arc::new(config); 

   
    for _i in 0..profile_value{
        let start = Instant::now(); 
        //Set up a new client session and TCP socket connection
        let mut sess = rustls::ClientSession::new(&arc_config, dns_name);
        let host_port = [&host, ":443"].join("");
        let mut sock = TcpStream::connect(&host_port).unwrap();
        let mut tls = rustls::Stream::new(&mut sess, &mut sock);
        

        //Send the http request through the socket
        let write_response = tls.write_all(http_request.as_bytes()); 
        match write_response {
            Ok(v) => println!("Message recieved while sending: {:?}", v),
            Err(e) => {
                println!("Recieved error while sending: {}\r\n", e);
                errors += 1; 
                continue; 
            }
        }

        //Read the http response from the server
        let mut buf = Vec::<u8>::new();
        let result = tls.read_to_end(&mut buf);
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
                println!("Recieved error while reading: {}", e);
                //CloseNotify alert not implemented universally in TLS libraries
                //Author chose this to be an error
                if e.to_string().contains("CloseNotify"){
                    println!("Peer asking to close TCP connection"); 
                    println!("Can still read server response"); 
                    let shutdown_result = sock.shutdown(Shutdown::Both); 
                    match shutdown_result{
                        Ok(v) => println!("Shutdown successful: {:?}", v),
                        Err(e) => println!("Failed to shutdown: {}", e)
                    }
                }
                else{
                    errors += 1; 
                    continue; 
                }
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
        println!("Message: {}", response_number); 
        let total_runtime = (old.duration_since(start).as_secs_f32()*1000.0).round() / 1000.0;
        println!("Status code: \r\n{}", response_number); 
        println!("Time for response: {:?}\r\n", total_runtime); 
        times.push(total_runtime); 
    }


    //Occurs when all request ended in errors and no bytes were ever read by client
    if big_size == 0{
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
    times.sort_by(|a, b| a.partial_cmp(b).unwrap()); 
    println!("Fastest time: {}s", times.first().unwrap()); 
    println!("Slowest time: {}s", times.last().unwrap()); 
    println!("Median time: {}s", (find_median(&times)*10000.0).round() / 10000.0); 
    println!("Mode time: {}s", find_mode(&times));
    println!("Percentage of requests succeeded: {}%", ((profile_value - errors) / profile_value) * 100); 
    println!("Size of smallest response: {} bytes", small_size); 
    println!("Size of biggest response: {} bytes", big_size); 
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