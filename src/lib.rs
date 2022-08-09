use std::sync::Arc;

use warp::filters::BoxedFilter;
use warp::http::HeaderValue;
use warp::Filter;

use regex::Regex;

/// Parse subdomain for warp
///
/// # Example:
/// ```use std::collections::HashMap;
///use std::sync::Arc;
///
///use warp::Filter;
///
///use warp_subdomain::with_subdomain;
///
///async fn query(
///    _query: HashMap<String, String>,
///    subdomain: Arc<Vec<String>>,
///) -> Result<impl warp::Reply, warp::Rejection> {
///    // get last subdomain
///    let subdomain_0 = subdomain.get(0);
///
///    Ok(warp::reply::with_status(
///        subdomain_0.unwrap().to_string(),
///        warp::http::StatusCode::FOUND,
///    ))
///}
///
///#[tokio::main]
///async fn main() {
///    let route = warp::path!("home")
///        .and(warp::get())
///        .and(warp::query::<HashMap<String, String>>())
///        .and(with_subdomain())
///        .and_then(query);
///
///    warp::serve(route).run(([127, 0, 0, 1], 3030)).await
///}
///```
pub fn with_subdomain() -> BoxedFilter<(Arc<Vec<String>>,)> {
    warp::header::value("host")
        .map(move |value: HeaderValue| {
            let init_value = format!("{:#?}", value);

            let strv = format!("{}", init_value);

            // split port if provided
            let splv: Vec<&str> = strv.split(":").collect();

            // remove port from host
            let clev = splv.first().unwrap();

            // split hostname
            let splv_2: Vec<&str> = clev.split(".").collect();

            let vl = splv_2.len();

            // regex to find " and \
            let rg = Regex::new("(\")|(\\\\)").unwrap();

            let mut sanv: Vec<String> = vec![];

            if vl > 2 {
                for (i, val) in splv_2.iter().enumerate() {
                    if i == (vl.wrapping_sub(2)) {
                        // Unused line due to domain name
                        /*let eoe = splv_2.get(vl.wrapping_sub(1)).unwrap();

                        let fv = rg.replace_all(&eoe, "");

                        sanv.push(format!("{}.{}", val, fv))*/
                    } else if i == vl.wrapping_sub(1) {
                    } else {
                        let fv = rg.replace_all(&val, "");

                        sanv.push(format!("{}", fv))
                    }
                }
            }

            Arc::<Vec<String>>::new(sanv).clone()
        })
        .boxed()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_filter_with_port() {
        let filter = with_subdomain();

        let value = warp::test::request()
            .path("/")
            .header("host", "super-alloy.api.cilen.com:3000")
            .filter(&filter)
            .await
            .unwrap();

        assert_eq!(
            value,
            Arc::new(vec!["super-alloy".to_string(), "api".to_string(),])
        );
    }

    #[tokio::test]
    async fn test_filter_without_port() {
        let filter = with_subdomain();

        let value = warp::test::request()
            .path("/")
            .header("host", "super-alloy.api.cilen.com")
            .filter(&filter)
            .await
            .unwrap();

        assert_eq!(
            value,
            Arc::new(vec!["super-alloy".to_string(), "api".to_string(),])
        );
    }

    #[tokio::test]
    async fn test_filter_without_subdomain() {
        let filter = with_subdomain();

        let value = warp::test::request()
            .path("/")
            .header("host", "cilen.com")
            .filter(&filter)
            .await
            .unwrap();

        assert_eq!(value, Arc::new(vec![]));
    }
}
