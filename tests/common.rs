
// NOTE: this macro can be used to generate a test for a specific endpoint.
#[macro_export]
macro_rules! test_http_endpoint {
    (
        test <$endpoint:path>
        with $method:ident $url:literal $($req_h_key:ident: $req_h_value:literal)* $req_body:literal
        and expect $code:literal $($res_h_key:ident: $res_h_value:literal)* $res_body:literal
    ) => {
        #[::actix_web::test]
        #[doc(hidden)]
        #[expect(non_snake_case)]
        async fn __test__$endpoint() {
            let listener = ::std::net::TcpListener::bind("127.0.0.1:0")
                .expect("Failed to bind random port.");
            let url = ::std::format!(
                "http://{}",
                listener.local_addr()
                    .expect("Failed to get local address.")
            );

            ::actix_web::HttpServer::new(|| {
                ::actix_web::App::new()
                    .service($endpoint)
            })
                .listen(listener)
                .expect("Failed to bind listener.")
                .run()
                .await
                .expect("Failed to run HTTP server.");

            let response = ::reqwest::Client::new()
                .$method($url)
                .headers(
                    ::reqwest::header::HeaderMap::from_iter([
                        $((
                            ::reqwest::header::HeaderName::from_static(::std::stringify!($req_h_key)),
                            ::reqwest::header::HeaderValue::from_static($req_h_value)
                        )),*
                    ])
                )
                .body($req_body)
                .send()
                .await
                .expect("Failed to send test request.");

            ::std::assert_eq!(response.status(), $code);

            {
                #[allow(unused)]
                let res_headers = a.headers();
                $(::std::assert_eq!(
                    res_headers.get(::std::stringify!($res_h_key)),
                    ::reqwest::header::HeaderValue::from_static($res_h_value)
                );)*
            }

            ::std::assert_eq!(response.body(), $res_body)

            ()
        }
    };
}
