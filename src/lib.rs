pub mod boat;
pub mod track;

use crate::boat::BoatStatus;

type CandelatteError = Box<dyn std::error::Error>;

const API_URL: &str = "https://app.candela.com";

pub trait BoatApi {
    fn boat_status(&self) -> Result<BoatStatus, CandelatteError>;
}
