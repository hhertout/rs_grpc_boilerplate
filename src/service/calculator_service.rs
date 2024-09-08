use crate::proto::{self, calculator_service_server::CalculatorService, AddRequest, AddResponse};

#[derive(Debug, Default)]
pub struct CalculatorSvc {}

#[tonic::async_trait]
impl CalculatorService for CalculatorSvc {
    async fn add(
        &self,
        request: tonic::Request<proto::AddRequest>,
    ) -> Result<tonic::Response<AddResponse>, tonic::Status> {
        println!("Got a add request");

        let input = request.get_ref();

        let response = AddResponse {
            result: input.a + input.b,
        };

        Ok(tonic::Response::new(response))
    }

    async fn divide(&self, request: tonic::Request<AddRequest>) -> Result<tonic::Response<AddResponse>, tonic::Status> {
        println!("Got a divide request");
        let input = request.get_ref();
        if input.b == 0 {
            return Err(tonic::Status::invalid_argument("Cannot divide by 0"))
        }
        let response = AddResponse {
            result: input.a / input.b
        };

        Ok(tonic::Response::new(response))
    }
}
