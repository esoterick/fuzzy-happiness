#![crate_name = "fuzzy_happiness"]
#![crate_type = "bin"]
#![allow(dead_code)]

extern crate hyper;
extern crate regex;
extern crate sqlite3;

use hyper::Client;
use hyper::header::Connection;
use regex::Regex;
use std::io::Read;
use std::string::String;

mod show;

fn main() {
    let mut client = Client::new();
    let mut show_list: Vec<show::Show> = Vec::new();
    let mut res = client.get("https://eztv.ch/")
        .header(Connection::close())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    get_shows(&body, &mut show_list);

    // cache_shows(&show_list);
    get_episodes(1012);
}

fn get_shows(src: &str, shows: &mut Vec<show::Show>) {
    let show_re = match Regex::new("<option value=\"([0-9]*)\">(.*)</option>") {
        Ok(show_re) => show_re,
        Err(err) => panic!("{}", err),
    };

    let caps = show_re.captures_iter(src);

    for cap in caps {
        if cap.at(1).unwrap_or("") != "" {
            let id: i64 = cap.at(1).unwrap().parse::<i64>().unwrap();
            let n: &str = cap.at(2).unwrap_or("");
            let mut nm: String = String::new();
            nm.push_str(n);
            shows.push(show::Show{id: id, name: nm });
        }
    }
}

// Silicon Valley 1012
fn get_episodes(id: i64) {
    let mut client = Client::new();

    // This is easier
    let show_id = id.to_string();
    let url = format!("https://eztv.ch/shows/{id}/",
                      id=&show_id);

    let mut res = client.get(&url)
        .header(Connection::close())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    let re = "<a href=\"(.*)\" title=\"(.*)\" alt=\"(.*)\" class=\"epinfo\">(.*)</a>";
    let show_re = match Regex::new(re) {
        Ok(show_re) => show_re,
        Err(err) => panic!("{}", err),
    };

    let caps = show_re.captures_iter(&body);

    for cap in caps {
        // 0 full
        // 1 url
        // 2 title
        // 3 alt
        // 4 contents
        println!("Episode: {}", cap.at(4).unwrap());
    }
    assert_eq!(res.status, hyper::Ok);
}

fn cache_shows(shows: & Vec<show::Show>) {
    for show in shows {
        println!("show {:?}", show);
    }
}
