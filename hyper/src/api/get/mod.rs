//! http://www.elastic.co/guide/en/elasticsearch/reference/master/docs-get.html

//Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn get_index_type_id<'a>(client: &'a mut Client, base: String, index: String,
                         _type: String, id: String) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 1 + index.len() +
                                  _type.len() + id.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&_type);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&id);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
