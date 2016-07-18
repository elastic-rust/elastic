use hyper::client::Client;
#[allow(unused_imports)]
use hyper::client::Body;
use hyper::client::response::Response;
use hyper::error::Result;

use ::RequestParams;

/// http://www.elasticsearch.org/guide/en/elasticsearch/reference/master/search-template.html
pub fn post_id<'a,
           I: Into<Body<'a>>>(client: &'a mut Client, req: &'a RequestParams,
                              id: &'a str, body: I) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 18 + id.len() + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_render/template/");
    url_fmtd.push_str(id);
    url_fmtd.push_str(url_qry);
    let res =
        client.post(&url_fmtd).headers(req.headers.to_owned()).body(body.into());
    res.send()
}

/// http://www.elasticsearch.org/guide/en/elasticsearch/reference/master/search-template.html
pub fn get<'a>(client: &'a mut Client, req: &'a RequestParams) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd = String::with_capacity(base.len() + 17 + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_render/template");
    url_fmtd.push_str(url_qry);
    let res = client.get(&url_fmtd).headers(req.headers.to_owned());
    res.send()
}

/// http://www.elasticsearch.org/guide/en/elasticsearch/reference/master/search-template.html
pub fn get_id<'a>(client: &'a mut Client, req: &'a RequestParams, id: &'a str)
 -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 18 + id.len() + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_render/template/");
    url_fmtd.push_str(id);
    url_fmtd.push_str(url_qry);
    let res = client.get(&url_fmtd).headers(req.headers.to_owned());
    res.send()
}

/// http://www.elasticsearch.org/guide/en/elasticsearch/reference/master/search-template.html
pub fn post<'a,
        I: Into<Body<'a>>>(client: &'a mut Client, req: &'a RequestParams,
                           body: I) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd = String::with_capacity(base.len() + 17 + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_render/template");
    url_fmtd.push_str(url_qry);
    let res =
        client.post(&url_fmtd).headers(req.headers.to_owned()).body(body.into());
    res.send()
}

