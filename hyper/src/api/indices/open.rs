//! http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-open-close.html

//Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

use RequestParams;

pub fn post_index<'a>(client: &'a mut Client, req: RequestParams, index: &'a str,
                  body: &'a str) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let mut url_fmtd =
        String::with_capacity(req.base_url.len() + 1 + 6 + index.len() +
                                  url_qry.len());
    url_fmtd.push_str(req.base_url);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/_open");
    url_fmtd.push_str(url_qry);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(body);
    res.send()
}
