
// NOTE: this macro can be used to generate a test for a specific endpoint.
#[macro_export]
macro_rules! test_http_endpoint {
    (
        test $endpoint:ident as $test_name:ident
        with $method:ident $url:literal $($req_h_key:ident: $req_h_value:literal)* $req_body:literal
        and expect $code:literal $($res_h_key:ident: $res_h_value:literal)* $($res_body:literal)?
    ) => {
        #[::actix_web::test]
        #[doc(hidden)]
        async fn $test_name() {
            let listener = ::std::net::TcpListener::bind("0.0.0.0:0")
                .expect("Failed to bind random port.");
            let local_addr = listener.local_addr()
                .expect("Failed to get local address.");
            let url = ::std::format!(
                "http://{local_addr}{}",
                $url
            );

            let server_thread = ::actix_web::rt::spawn(async move {
                ::actix_web::HttpServer::new(|| {
                    ::actix_web::App::new()
                        .service($endpoint)
                })
                    .listen(listener)
                    .expect("Failed to bind listener.")
                    .run()
                    .await
                    .expect("Failed to run HTTP server.");
            });

            loop {
                if ::std::net::TcpStream::connect(local_addr).is_ok() {
                    break;
                }

                ::std::thread::sleep(::std::time::Duration::from_millis(500))
            }

            let response = ::reqwest::Client::new()
                .$method(url)
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
                let res_headers = response.headers();
                $(::std::assert_eq!(
                    res_headers.get(::std::stringify!($res_h_key)),
                    Some(&::reqwest::header::HeaderValue::from_static($res_h_value))
                );)*
            }

            $(
                ::std::assert_eq!(
                    response.text()
                        .await
                        .expect("Expected a response body"),
                    $res_body
                );
            )?

            server_thread.abort();

            ()
        }
    };
}
