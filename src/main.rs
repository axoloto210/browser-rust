#![no_std]  // 標準ライブラリを使用しない
#![no_main] // 標準のmain関数エントリーポイントを使用しない
#![cfg_attr(not(target_os = "linux"), no_main)]

extern crate alloc; //no_stdのため、明示的にallocを導入する必要がある

use alloc::string::ToString;
use net_wasabi::http::HttpClient;

use noli::prelude::*;

fn main() {
    let client = HttpClient::new();
    match client.get("example.com".to_string(), 80, "/".to_string()){
        Ok(res)=>{
            print!("response:\n{:#?}",res);

        }
        Err(e)=>{
            print!("error:\n{:#?}",e)
        }
    }
}

entry_point!(main);