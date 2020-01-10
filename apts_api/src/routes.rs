use apts_api::examples;

#[get("/main")]
pub fn index() -> &'static str {
  "Application successfully started!"
}

#[get("/payments")]
pub fn run_payments() -> &'static str {
  examples::run_payments();
  "Payments api call successfully called!"
}