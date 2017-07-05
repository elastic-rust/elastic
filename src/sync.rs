use std::io::Cursor;
use std::fs::File;
use serde::de::DeserializeOwned;
use reqwest::{self, RequestBuilder, Response};

use super::req::HttpRequest;
use super::res::parsing::{Parse, IsOk};
use super::res::error::ResponseError;
use super::{RequestParams, build_url, build_method};

/// A type that can be converted into a request body.
pub trait IntoBodySync {
    /// Convert self into a body.
    fn into_body(self) -> reqwest::Body;
}

impl IntoBodySync for reqwest::Body {
    fn into_body(self) -> reqwest::Body {
        self
    }
}

impl IntoBodySync for File {
    fn into_body(self) -> reqwest::Body {
        self.into()
    }
}

impl IntoBodySync for Vec<u8> {
    fn into_body(self) -> reqwest::Body {
        self.into()
    }
}

impl IntoBodySync for String {
    fn into_body(self) -> reqwest::Body {
        self.into()
    }
}

impl IntoBodySync for &'static [u8] {
    fn into_body(self) -> reqwest::Body {
        reqwest::Body::new(Cursor::new(self))
    }
}

impl IntoBodySync for &'static str {
    fn into_body(self) -> reqwest::Body {
        reqwest::Body::new(Cursor::new(self))
    }
}

/// Represents a client that can send Elasticsearch requests.
pub trait ElasticClientSync {
    /// Send a request and get a response.
    ///
    /// # Examples
    ///
    /// Bring the `ElasticClientSync` trait into scope and call `elastic_req` with any type that
    /// can be converted into a `req::HttpRequest`.
    /// This method returns a raw `reqwest::Response`.
    ///
    /// ```no_run
    /// # use elastic_reqwest::req::SimpleSearchRequest;
    /// # let request = SimpleSearchRequest::for_index_ty("myindex", "mytype");
    /// use elastic_reqwest::ElasticClientSync;
    ///
    /// let (client, params) = elastic_reqwest::default().unwrap();
    ///
    /// let http_res = client.elastic_req(&params, request).unwrap();
    /// ```
    fn elastic_req<I, B>(&self, params: &RequestParams, req: I) -> Result<Response, reqwest::Error>
        where I: Into<HttpRequest<'static, B>>,
              B: IntoBodySync;
}

/// Represents a response that can be parsed into a concrete Elasticsearch response.
pub trait ParseResponse<TResponse> {
    /// Parse a response into a concrete response type.
    fn from_response(self, response: Response) -> Result<TResponse, ResponseError>;
}

impl<TResponse: IsOk + DeserializeOwned> ParseResponse<TResponse> for Parse<TResponse> {
    fn from_response(self, response: Response) -> Result<TResponse, ResponseError> {
        let status: u16 = response.status().into();

        self.from_reader(status, response)
    }
}

fn build_req<I, B>(client: &reqwest::Client, params: &RequestParams, req: I) -> Result<RequestBuilder, reqwest::Error>
    where I: Into<HttpRequest<'static, B>>,
          B: IntoBodySync
{
    let req = req.into();

    let url = build_url(&req.url, &params);
    let method = build_method(req.method);
    let body = req.body;

    let mut req = client.request(method, &url)?;
    {
        req.headers(params.headers.to_owned());

        if let Some(body) = body {
            req.body(body.into_body());
        }
    }

    Ok(req)
}

impl ElasticClientSync for reqwest::Client {
    fn elastic_req<I, B>(&self, params: &RequestParams, req: I) -> Result<Response, reqwest::Error>
        where I: Into<HttpRequest<'static, B>>,
              B: IntoBodySync
    {
        build_req(&self, params, req)?.send()
    }
}

#[cfg(test)]
mod tests {
    use reqwest::{Client, RequestBuilder, Method};
    use reqwest::header::ContentType;
    use super::*;
    use req::*;

    fn params() -> RequestParams {
        RequestParams::new("eshost:9200/path")
            .url_param("pretty", true)
            .url_param("q", "*")
    }

    fn expected_req(cli: &Client, method: Method, url: &str, body: Option<Vec<u8>>) -> RequestBuilder {
        let mut req = cli.request(method, url).unwrap();
        {
            req.header(ContentType::json());

            if let Some(body) = body {
                req.body(body);
            }
        }

        req
    }

    fn assert_req(expected: RequestBuilder, actual: RequestBuilder) {
        assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
    }

    #[test]
    fn head_req() {
        let cli = Client::new().unwrap();
        let req = build_req(&cli, &params(), PingHeadRequest::new());

        let url = "eshost:9200/path/?pretty=true&q=*";

        let expected = expected_req(&cli, Method::Head, url, None);

        assert_req(expected, req.unwrap());
    }

    #[test]
    fn get_req() {
        let cli = Client::new().unwrap();
        let req = build_req(&cli, &params(), SimpleSearchRequest::new());

        let url = "eshost:9200/path/_search?pretty=true&q=*";

        let expected = expected_req(&cli, Method::Get, url, None);

        assert_req(expected, req.unwrap());
    }

    #[test]
    fn post_req() {
        let cli = Client::new().unwrap();
        let req = build_req(&cli,
                            &params(),
                            PercolateRequest::for_index_ty("idx", "ty", vec![]));

        let url = "eshost:9200/path/idx/ty/_percolate?pretty=true&q=*";

        let expected = expected_req(&cli, Method::Post, url, Some(vec![]));

        assert_req(expected, req.unwrap());
    }

    #[test]
    fn put_req() {
        let cli = Client::new().unwrap();
        let req = build_req(&cli,
                            &params(),
                            IndicesCreateRequest::for_index("idx", vec![]));

        let url = "eshost:9200/path/idx?pretty=true&q=*";

        let expected = expected_req(&cli, Method::Put, url, Some(vec![]));

        assert_req(expected, req.unwrap());
    }

    #[test]
    fn delete_req() {
        let cli = Client::new().unwrap();
        let req = build_req(&cli, &params(), IndicesDeleteRequest::for_index("idx"));

        let url = "eshost:9200/path/idx?pretty=true&q=*";

        let expected = expected_req(&cli, Method::Delete, url, None);

        assert_req(expected, req.unwrap());
    }

    #[test]
    fn file_into_body() {
        File::open("Cargo.toml").unwrap().into_body();
    }

    #[test]
    fn owned_string_into_body() {
        String::new().into_body();
    }

    #[test]
    fn borrowed_string_into_body() {
        "abc".into_body();
    }

    #[test]
    fn owned_vec_into_body() {
        Vec::new().into_body();
    }

    #[test]
    fn borrowed_vec_into_body() {
        static BODY: &'static [u8] = &[0, 1, 2];

        (&BODY).into_body();
    }
}