use hyper::client::Client;
#[allow(unused_imports)]
use hyper::client::Body;
use hyper::client::response::Response;
use hyper::error::Result;

use ::RequestParams;

/// http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-stats.html
pub fn get_metric_index_metric<'a>(client: &'a mut Client, req: &'a RequestParams,
                               metric: &'a str, index_metric: &'a str)
 -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 14 + 1 + metric.len() +
                                  index_metric.len() + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_nodes/stats/");
    url_fmtd.push_str(metric);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index_metric);
    url_fmtd.push_str(url_qry);
    let res = client.get(&url_fmtd).headers(req.headers.to_owned());
    res.send()
}

/// http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-stats.html
pub fn get_metric<'a>(client: &'a mut Client, req: &'a RequestParams,
                  metric: &'a str) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 14 + metric.len() + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_nodes/stats/");
    url_fmtd.push_str(metric);
    url_fmtd.push_str(url_qry);
    let res = client.get(&url_fmtd).headers(req.headers.to_owned());
    res.send()
}

/// http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-stats.html
pub fn get<'a>(client: &'a mut Client, req: &'a RequestParams) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd = String::with_capacity(base.len() + 13 + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_nodes/stats");
    url_fmtd.push_str(url_qry);
    let res = client.get(&url_fmtd).headers(req.headers.to_owned());
    res.send()
}

/// http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-stats.html
pub fn get_node_id_metric_index_metric<'a>(client: &'a mut Client,
                                       req: &'a RequestParams,
                                       node_id: &'a str, metric: &'a str,
                                       index_metric: &'a str)
 -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 8 + 7 + 1 + node_id.len() +
                                  metric.len() + index_metric.len() +
                                  url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_nodes/");
    url_fmtd.push_str(node_id);
    url_fmtd.push_str("/stats/");
    url_fmtd.push_str(metric);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index_metric);
    url_fmtd.push_str(url_qry);
    let res = client.get(&url_fmtd).headers(req.headers.to_owned());
    res.send()
}

/// http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-stats.html
pub fn get_node_id_metric<'a>(client: &'a mut Client, req: &'a RequestParams,
                          node_id: &'a str, metric: &'a str)
 -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 8 + 7 + node_id.len() +
                                  metric.len() + url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_nodes/");
    url_fmtd.push_str(node_id);
    url_fmtd.push_str("/stats/");
    url_fmtd.push_str(metric);
    url_fmtd.push_str(url_qry);
    let res = client.get(&url_fmtd).headers(req.headers.to_owned());
    res.send()
}

/// http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-nodes-stats.html
pub fn get_node_id<'a>(client: &'a mut Client, req: &'a RequestParams,
                   node_id: &'a str) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 8 + 6 + node_id.len() +
                                  url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_nodes/");
    url_fmtd.push_str(node_id);
    url_fmtd.push_str("/stats");
    url_fmtd.push_str(url_qry);
    let res = client.get(&url_fmtd).headers(req.headers.to_owned());
    res.send()
}

