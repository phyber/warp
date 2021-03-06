#![deny(warnings)]
extern crate warp;

use warp::Filter;

#[test]
fn method() {
    let get = warp::get(warp::any().map(warp::reply));

    let req = warp::test::request();
    assert!(req.matches(&get));

    let req = warp::test::request()
        .method("POST");
    assert!(!req.matches(&get));

    let req = warp::test::request()
        .method("POST");
    let resp = req.reply(&get);
    assert_eq!(resp.status(), 405);
}

#[test]
fn method_not_allowed_trumps_not_found() {
    let get = warp::get(warp::path("hello").map(warp::reply));
    let post = warp::post(warp::path("bye").map(warp::reply));

    let routes = get.or(post);


    let req = warp::test::request()
        .method("GET")
        .path("/bye");

    let resp = req.reply(&routes);
    // GET was allowed, but only for /hello, so POST returning 405 is fine.
    assert_eq!(resp.status(), 405);
}

#[test]
fn bad_request_trumps_method_not_allowed() {
    let get = warp::get(
        warp::path("hello")
            .and(warp::header::exact("foo", "bar"))
            .map(warp::reply)
    );
    let post = warp::post(
        warp::path("bye").map(warp::reply)
    );

    let routes = get.or(post);


    let req = warp::test::request()
        .method("GET")
        .path("/hello");

    let resp = req.reply(&routes);
    // GET was allowed, but header rejects with 400, should not
    // assume POST was the appropriate method.
    assert_eq!(resp.status(), 400);
}

