//Autogenerated
use hyper::Client;
pub fn get_repository_snapshot(base: String, repository: String, snapshot: String)
 -> String{
    let mut url_fmtd =
        String::with_capacity(base.len() + 11 + 1 + repository.len() +
                                  snapshot.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_snapshot/");
    url_fmtd.push_str(&repository);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&snapshot);
    url_fmtd
}
