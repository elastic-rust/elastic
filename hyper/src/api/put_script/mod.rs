//! http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-scripting.html

//Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn put_lang_id<'a>(client: &'a mut Client, base: &'a str, lang: &'a str,
                   id: &'a str, body: &'a str) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 10 + 1 + lang.len() + id.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_scripts/");
    url_fmtd.push_str(lang);
    url_fmtd.push_str("/");
    url_fmtd.push_str(id);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.put(&url_fmtd).headers(headers).body(body);
    res.send()
}
pub fn post_lang_id<'a>(client: &'a mut Client, base: &'a str, lang: &'a str,
                    id: &'a str, body: &'a str) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 10 + 1 + lang.len() + id.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_scripts/");
    url_fmtd.push_str(lang);
    url_fmtd.push_str("/");
    url_fmtd.push_str(id);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(body);
    res.send()
}
