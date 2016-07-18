//! http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-shrink-index.html

//Autogenerated

use hyper::client::Client;
#[allow(unused_imports)]
use hyper::client::Body;
use hyper::client::response::Response;
use hyper::error::Result;

use ::RequestParams;

pub fn put_index_target<'a,
                    I: Into<Body<'a>>>(client: &'a mut Client,
                                       req: &'a RequestParams, index: &'a str,
                                       target: &'a str, body: I)
 -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 9 + index.len() + target.len()
                                  + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/_shrink/");
    url_fmtd.push_str(target);
    url_fmtd.push_str(url_qry);
    let res =
        client.put(&url_fmtd).headers(req.headers.to_owned()).body(body.into());
    res.send()
}
pub fn post_index_target<'a,
                     I: Into<Body<'a>>>(client: &'a mut Client,
                                        req: &'a RequestParams,
                                        index: &'a str, target: &'a str,
                                        body: I) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 9 + index.len() + target.len()
                                  + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/_shrink/");
    url_fmtd.push_str(target);
    url_fmtd.push_str(url_qry);
    let res =
        client.post(&url_fmtd).headers(req.headers.to_owned()).body(body.into());
    res.send()
}
