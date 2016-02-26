//Autogenerated

use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn get(client: &'a mut hyper::Client, base: String) -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 14);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_cat/recovery");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_index(client: &'a mut hyper::Client, base: String, index: String)
 -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 15 + index.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_cat/recovery/");
    url_fmtd.push_str(&index);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
