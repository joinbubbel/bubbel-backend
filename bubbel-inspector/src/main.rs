use colored_json::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::BufRead;

#[derive(Serialize, Deserialize)]
pub struct InDebug {
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResDebugState {
    incoming: Vec<Option<(u64, String)>>,
    outgoing: Vec<Option<(u64, String)>>,
}

fn main() {
    let Some(base_url) = std::env::args().nth(1) else {
        println!("Expected base url eg: 'https://mybackend.com'");
        return;
    };

    println!("Enter password: ");
    let mut password = String::new();
    let mut stdin = std::io::stdin().lock();
    stdin.read_line(&mut password).unwrap();
    password.pop();

    let mut used_ids = vec![];
    let req = InDebug { password };
    let url = base_url + "/api/debug";

    loop {
        let res: Option<ResDebugState> = reqwest::blocking::Client::new()
            .post(&url)
            .json(&req)
            .send()
            .unwrap()
            .json()
            .ok();

        if let Some(res) = res {
            res.incoming.iter().for_each(|o| {
                if let Some((id, s)) = o {
                    if !used_ids.iter().any(|t| t == id) {
                        println!("<========== Incoming ==========>\n");
                        println!("{}\n", s.to_colored_json_auto().unwrap());
                        used_ids.push(*id);
                        clamp_vec(&mut used_ids);
                    }
                }
            });
            res.outgoing.iter().for_each(|o| {
                if let Some((id, s)) = o {
                    if !used_ids.iter().any(|t| t == id) {
                        println!("<========== Outgoing ==========>\n");
                        println!("{}\n", s.to_colored_json_auto().unwrap());
                        used_ids.push(*id);
                        clamp_vec(&mut used_ids);
                    }
                }
            });
        } else {
            println!("Got error. Either connection dropped, password wrong, or backend debug not enabled.");
            return;
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn clamp_vec<T>(v: &mut Vec<T>) {
    if v.len() > 20 {
        v.remove(0);
    }
}
