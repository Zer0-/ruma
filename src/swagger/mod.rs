//! Data for [Swagger UI](https://github.com/swagger-api/swagger-ui).

use iron::{Chain, Handler, IronResult, Request, Response, status};
use iron::headers::ContentType;
use iron::modifiers::Header;
use mount::Mount;

use middleware::Cors;

#[cfg(not(feature = "swagger"))]
pub fn mount_swagger(_mount: &mut Mount) {}

#[cfg(feature = "swagger")]
pub fn mount_swagger(mount: &mut Mount) {
    struct Swagger;

    impl Handler for Swagger {
        fn handle(&self, _request: &mut Request) -> IronResult<Response> {
            let json = include_str!("swagger.json");

            Ok(Response::with((
                status::Ok,
                Header(ContentType::json()),
                json,
            )))
        }
    }

    let mut swagger = Chain::new(Swagger);

    swagger.link_after(Cors);

    mount.mount("/ruma/swagger.json", swagger);
}