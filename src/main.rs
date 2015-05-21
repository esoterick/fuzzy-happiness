extern crate hyper;
extern crate regex;

use std::io::Read;
use hyper::Client;
use hyper::header::Connection;
use regex::Regex;
use std::string::String;

#[derive(Debug)]
struct Show {
    id: i32,
    name: String,
}

fn main() {
    let mut client = Client::new();
    let mut show_list: Vec<Show> = Vec::new();
    let mut res = client.get("https://eztv.ch/")
        .header(Connection::close())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    get_shows(& body, & mut show_list);

    println!("shows: {:?}", show_list);
}

fn get_shows(src: & str, shows: & mut Vec<Show>) {
    let show_re = match Regex::new("<option value=\"([0-9]*)\">(.*)</option>") {
        Ok(show_re) => show_re,
        Err(err) => panic!("{}", err),
    };

    let caps = show_re.captures_iter(src);

    for cap in caps {
        if cap.at(1).unwrap_or("") != "" {
            let id: i32 = cap.at(1).unwrap().parse::<i32>().unwrap();
            let n: &str = cap.at(2).unwrap_or("");
            let mut nm: String = String::new();
            nm.push_str(n);
            shows.push(Show{id: id, name: nm });
        }
    }
}