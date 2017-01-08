extern crate chrono;
extern crate serde_json;

use chrono::NaiveDateTime;
use serde_json::Value;
use std::process::Command;


fn main() {
    show();
}


fn show() {
    let output = Command::new("zcash-cli").arg("getbalance").output().expect("failed to execute zcash-cli").stdout;
    let balance = std::str::from_utf8(&output).unwrap().trim().parse::<f32>().expect("parsing balace as float");

    let output = Command::new("zcash-cli").arg("listtransactions").output().expect("failed to execute zcash-cli").stdout;
    let data: Value = serde_json::from_str(std::str::from_utf8(&output).unwrap()).unwrap();


    println!("Recent Transactions:");
    for transactions in data.as_array() {
        for txn in transactions {
            //println!("{:?}", txn);
            let received_time = NaiveDateTime::from_timestamp(txn.find("timereceived").unwrap().as_i64().unwrap(), 0);
            println!("  {} {:.8} to {:.5} at {}",
                txn.find("category").unwrap().as_str().unwrap(),
                txn.find("amount").unwrap().as_f64().unwrap(),
                txn.find("address").unwrap().as_str().unwrap(),
                received_time.format("%b %d %H:%M:%S"));
        }
    }

    println!("\nBalance:\n  {} ZEC", balance);
}