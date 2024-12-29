#![no_std]  // 標準ライブラリを使用しない
#![no_main] // 標準のmain関数エントリーポイントを使用しない

extern crate alloc; //no_stdのため、明示的にallocを導入する必要がある

use alloc::string::ToString;
use net_wasabi::http::HttpClient;

use noli::prelude::*;

fn main()->u64 {
    let client = HttpClient::new();
    match client.get("host.test".to_string(), 8000, "/test.html".to_string()){
        Ok(res)=>{
            print!("response:\n{:#?}",res);

        }
        Err(e)=>{
            print!("error:\n{:#?}",e)
        }
    }
    0
}

entry_point!(main);