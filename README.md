# Subdomain-warp

A simple subdomain parser middleware for [Warp](https://github.com/seanmonstar/warp).

## Usage

```
... warp route
.and(warp_subdomain::with_subdomain)`
... route handler
```

## Example

```
use std::collections::HashMap;
use std::sync::Arc;

use warp::Filter;

use warp_subdomain::with_subdomain;

async fn query(
    _query: HashMap<String, String>,
    subdomain: Arc<Vec<String>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // get last subdomain
    let subdomain_0 = subdomain.get(0);

    Ok(warp::reply::with_status(
        subdomain_0.unwrap().to_string(),
        warp::http::StatusCode::FOUND,
    ))
}

#[tokio::main]
async fn main() {
    let route = warp::path!("home")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and(with_subdomain())
        .and_then(query);

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await
}

```

If host eg. <strong>super-alloy.api.cilen.com.</strong>
<br/>
This middleware will return `Arc::Vec<String> = vec!["super-alloy", "api"]`

This middleware also works for localhost that have port in host header. eg. <strong>api.localhost:3999</strong>

## Note

This middlewares will return value with type `Arc<Vec<String>>`

## Misc

- [API Documentation]()
- [Examples]()
