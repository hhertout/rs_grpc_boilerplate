use std::env;

use tonic::{service::Interceptor, Request, Status};

#[derive(Default, Clone)]
pub struct ApiKeyInterceptor;

impl Interceptor for ApiKeyInterceptor {
    fn call(&mut self, req: Request<()>) -> Result<Request<()>, Status> {
        let token = env::var("API_KEY").expect("API KEY IS NOT SET");
        let headers = req.metadata().clone().into_headers();

        let req_token = headers.get("x-api-key");
        log::debug!("req_api_token={:?}", req_token);

        match req_token {
            Some(t) => {
                let provided_token = t
                    .to_str()
                    .map_err(|err| tonic::Status::internal(err.to_string()))
                    .unwrap();

                if token != provided_token {
                    return Err(tonic::Status::permission_denied("INVALID API KEY"));
                }
                Ok(req)
            }
            None => Err(tonic::Status::permission_denied("INVALID API KEY")),
        }
    }
}
