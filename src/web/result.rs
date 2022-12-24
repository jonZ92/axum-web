use derivative::Derivative;
use serde::Serialize;

#[warn(legacy_derive_helpers)]
#[derivative(Default)]
#[derive(Derivative, Serialize)]
pub struct Result <T>{
    #[derivative(Default(value = "0"))]
    pub code: u32,

    #[derivative(Default(value="true"))]
    pub status: bool,

    pub data:Option<T>,
}
