//! http://www.elasticsearch.org/guide/en/elasticsearch/reference/master/search-template.html

// Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn get_id<'a>(client: &'a mut Client, base: &'a str, id: &'a str) -> Result<Response> {
    let mut url_fmtd = String::with_capacity(base.len() + 18 + id.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_render/template/");
    url_fmtd.push_str(id);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn post_id<'a>(client: &'a mut Client,
                   base: &'a str,
                   id: &'a str,
                   body: String)
                   -> Result<Response> {
    let mut url_fmtd = String::with_capacity(base.len() + 18 + id.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_render/template/");
    url_fmtd.push_str(id);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(&body);
    res.send()
}
pub fn post<'a>(client: &'a mut Client, base: &'a str, body: String) -> Result<Response> {
    let mut url_fmtd = String::with_capacity(base.len() + 17);
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_render/template");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(&body);
    res.send()
}
pub fn get<'a>(client: &'a mut Client, base: &'a str) -> Result<Response> {
    let mut url_fmtd = String::with_capacity(base.len() + 17);
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_render/template");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
