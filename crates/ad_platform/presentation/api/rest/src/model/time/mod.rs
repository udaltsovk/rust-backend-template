use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema, Validate, Debug)]
///
pub struct JsonTime {
    #[schema(format = UInt32, default = 1, minimum = 1)]
    #[garde(range(min = 1))]
    /// Текущий день (целое число).
    pub current_date: i64,
}
