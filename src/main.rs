extern crate chrono;
extern crate serde_json;

use chrono::NaiveDateTime;
use serde_json::Value;
use std::process::Command;


fn main() {
    show();
}

fn show() -> bool {
    // we only bother with the error checking the first time, because if it's up it's up
    let balance = match Command::new("zcash-cli").arg("getbalance").output() {
        Ok(out) => {
            let stdout = String::from_utf8(out.stdout).expect("stdout not UTF8");
            if stdout != "" {
                stdout.trim().parse::<f32>().unwrap()
            } else {
                // zcashd not ready yet
                println!("{}", String::from_utf8(out.stderr).expect("stderr not UTF8"));
                return false;
            }
        },
        Err(e) => {
            println!("Failed to run zcash-cli: {}", e);
            return false;
        }
    };

    let output = Command::new("zcash-cli").arg("getunconfirmedbalance").output().expect("failed to execute zcash-cli").stdout;
    let unconfirmed_balance = std::str::from_utf8(&output).unwrap().trim().parse::<f32>().expect("parsing unc-balace as float");

    let output = Command::new("zcash-cli").arg("listtransactions").output().expect("failed to execute zcash-cli").stdout;
    let data: Value = serde_json::from_str(std::str::from_utf8(&output).unwrap()).unwrap();

    println!("Recent Transactions:");
    for transactions in data.as_array() {
        for txn in transactions {
            //println!("{:?}", txn);
            let received_time = match txn.find("blocktime") {
                Some(string) => NaiveDateTime::from_timestamp(string.as_i64().unwrap(), 0).format("%b %d %H:%M:%S").to_string(),
                None => "---------------".to_string()
            };
            println!("  {}    {:.8} ZEC    (to {:.7} in txn {:.7})",
                received_time,
                txn.find("amount").unwrap().as_f64().unwrap(),
                txn.find("address").unwrap().as_str().unwrap(),
                txn.find("txid").unwrap().as_str().unwrap()
            );
        }
    }

    println!("\nConfirmed Balance:   {:.8} ZEC", balance);
    if unconfirmed_balance > 0.0 {
        println!("\nUnconfirmed Balance: {:.8} ZEC", unconfirmed_balance);
        println!("Total Balance:       {:.8} ZEC", balance + unconfirmed_balance);
    }
    return true;
}
