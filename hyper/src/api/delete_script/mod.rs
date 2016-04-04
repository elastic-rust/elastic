//! http://www.elastic.co/guide/en/elasticsearch/reference/master/modules-scripting.html

//Autogenerated

use hyper::client::Client;
use hyper::client::response::Response;
use hyper::error::Result;

use RequestParams;

pub fn delete_lang_id<'a>(client: &'a mut Client, req: RequestParams,
                      lang: &'a str, id: &'a str) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 10 + 1 + lang.len() + id.len() +
                                  url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_scripts/");
    url_fmtd.push_str(lang);
    url_fmtd.push_str("/");
    url_fmtd.push_str(id);
    url_fmtd.push_str(url_qry);
    let res = client.delete(&url_fmtd).headers(req.headers);
    res.send()
}
