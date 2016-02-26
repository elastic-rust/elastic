//Autogenerated
pub fn delete_index_name(base: String, index: String, name: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 8 + index.len() + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_alias/");
    url_fmtd.push_str(&name);
    url_fmtd
}
pub fn delete_index_name(base: String, index: String, name: String) -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 10 + index.len() + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_aliases/");
    url_fmtd.push_str(&name);
    url_fmtd
}
