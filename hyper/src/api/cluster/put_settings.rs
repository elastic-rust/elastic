//! http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-update-settings.html

//Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

use RequestParams;

pub fn put<'a>(client: &'a mut Client, req: RequestParams, base: &'a str,
           body: &'a str) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let mut url_fmtd = String::with_capacity(base.len() + 18 + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_cluster/settings");
    url_fmtd.push_str(url_qry);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.put(&url_fmtd).headers(headers).body(body);
    res.send()
}
