//! http://www.elastic.co/guide/en/elasticsearch/reference/master/cat-shards.html

//Autogenerated

use hyper::client::Client;
use hyper::client::response::Response;
use hyper::error::Result;

use RequestParams;

pub fn get_index<'a>(client: &'a mut Client, req: RequestParams, index: &'a str)
 -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 13 + index.len() + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_cat/shards/");
    url_fmtd.push_str(index);
    url_fmtd.push_str(url_qry);
    let res = client.get(&url_fmtd).headers(req.headers);
    res.send()
}
pub fn get<'a>(client: &'a mut Client, req: RequestParams) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd = String::with_capacity(base.len() + 12 + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_cat/shards");
    url_fmtd.push_str(url_qry);
    let res = client.get(&url_fmtd).headers(req.headers);
    res.send()
}
