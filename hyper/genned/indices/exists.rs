//Autogenerated
use hyper::Client;
pub fn head_index(base: String, index: String) -> String{
    let mut url_fmtd = String::with_capacity(base.len() + 1 + index.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd
}
