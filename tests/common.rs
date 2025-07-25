
// NOTE: this macro can be used to generate a test for a specific endpoint.
#[macro_export]
macro_rules! test_http_endpoint {
    (
        test $endpoint:ident as $test_name:ident
        with request {
            head: $req_method:ident /$($($req_path_segment:ident)/+)? $(?$($req_query_key:ident=$req_query_value:literal)&*)?;
            $(headers: { $($req_header_key:ident: $req_header_value:literal)* })?
            $(body: { $req_body:expr })?
        }
        and expect response {
            head: $res_code:literal;
            $(headers: { $($res_header_key:ident: $res_header_value:literal)* })?
            $(body: { $res_body:expr })?
        }
    ) => {
        #[::actix_web::test]
        #[doc(hidden)]
        async fn $test_name() {
            #[allow(unused_imports)]
            use std::fmt::Write as _;

            let listener = ::std::net::TcpListener::bind("0.0.0.0:0")
                .expect("Failed to bind random port.");
            let local_addr = listener.local_addr()
                .expect("Failed to get local address.");
            let mut url = ::std::format!("http://{local_addr}");
            $($(
                ::std::write!(url, "/{}", ::std::stringify!($req_path_segment))
                    .unwrap();
            )*)?
            ::std::write!(url, "?")
                .unwrap();
            $($(
                ::std::write!(url, "{}={}", ::std::stringify!($req_query_key), $req_query_value)
                    .unwrap();
            )*)?
            println!("{url}");

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
                .$req_method(url)
                .headers(
                    ::reqwest::header::HeaderMap::from_iter([
                        $((
                            ::reqwest::header::HeaderName::from_static(::std::stringify!($req_header_key)),
                            ::reqwest::header::HeaderValue::from_static($req_header_value)
                        )),*
                    ])
                )
                $(
                    .body($req_body)
                )?
                .send()
                .await
                .expect("Failed to send test request.");

            ::std::assert_eq!(response.status(), $res_code);

            $({
                #[allow(unused)]
                let res_headers = response.headers();
                $(::std::assert_eq!(
                    res_headers.get(::std::stringify!($res_header_key)),
                    Some(&::reqwest::header::HeaderValue::from_static($res_header_value))
                );)*
            })?

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
