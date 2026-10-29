use serde::Serialize;

#[cfg(test)]
use serde::Deserialize;

#[derive(Serialize)]
#[cfg_attr(test, derive(Deserialize, PartialEq, Debug))]
pub struct CsrfResponse {
    pub csrf_token: String,
}
