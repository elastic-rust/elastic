//! http://www.elastic.co/guide/en/elasticsearch/reference/master/search-validate.html

//Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn get<'a>(client: &'a mut Client, base: String) -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 16);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_validate/query");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn post_index<'a>(client: &'a mut Client, base: String, index: String,
                  body: String) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 16 + index.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_validate/query");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(&body);
    res.send()
}
pub fn post<'a>(client: &'a mut Client, base: String, body: String)
 -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 16);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_validate/query");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(&body);
    res.send()
}
pub fn get_index<'a>(client: &'a mut Client, base: String, index: String)
 -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 16 + index.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_validate/query");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_index_type<'a>(client: &'a mut Client, base: String, index: String,
                      _type: String) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 16 + index.len() +
                                  _type.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&_type);
    url_fmtd.push_str("/_validate/query");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn post_index_type<'a>(client: &'a mut Client, base: String, index: String,
                       _type: String, body: String) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 16 + index.len() +
                                  _type.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&_type);
    url_fmtd.push_str("/_validate/query");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(&body);
    res.send()
}
