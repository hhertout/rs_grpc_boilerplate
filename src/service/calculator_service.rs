use crate::proto::{self, calculator_server::Calculator, CalculationRequest, CalculationResponse};

#[derive(Debug, Default)]
pub struct CalculatorService {}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
    ) -> Result<tonic::Response<CalculationResponse>, tonic::Status> {
        println!("Got a add request");

        let input = request.get_ref();

        let response = CalculationResponse {
            result: input.a + input.b,
        };

        Ok(tonic::Response::new(response))
    }

    async fn divide(&self, request: tonic::Request<CalculationRequest>) -> Result<tonic::Response<CalculationResponse>, tonic::Status> {
        println!("Got a divide request");
        let input = request.get_ref();
        if input.b == 0 {
            return Err(tonic::Status::invalid_argument("Cannot divide by 0"))
        }
        let reponse = CalculationResponse {
            result: input.a / input.b
        };

        Ok(tonic::Response::new(reponse))
    }
}
