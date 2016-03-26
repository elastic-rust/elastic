//! http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-synced-flush.html

//Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

use RequestParams;

pub fn post_index<'a>(client: &'a mut Client, req: RequestParams, base: &'a str,
                  index: &'a str, body: &'a str) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 14 + index.len() +
                                  url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/_flush/synced");
    url_fmtd.push_str(url_qry);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(body);
    res.send()
}
pub fn post<'a>(client: &'a mut Client, req: RequestParams, base: &'a str,
            body: &'a str) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let mut url_fmtd = String::with_capacity(base.len() + 14 + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_flush/synced");
    url_fmtd.push_str(url_qry);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(body);
    res.send()
}
pub fn get_index<'a>(client: &'a mut Client, req: RequestParams, base: &'a str,
                 index: &'a str) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 14 + index.len() +
                                  url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/_flush/synced");
    url_fmtd.push_str(url_qry);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get<'a>(client: &'a mut Client, req: RequestParams, base: &'a str)
 -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let mut url_fmtd = String::with_capacity(base.len() + 14 + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_flush/synced");
    url_fmtd.push_str(url_qry);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
