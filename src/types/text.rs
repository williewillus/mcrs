use serde::{Deserialize, Serialize};

// TODO: temporary for status check.

#[derive(Deserialize, Serialize)]
pub struct Text {
    pub text: String
}
