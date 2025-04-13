// #![cfg(feature = "std")]

use sp_std::prelude::*;

use sp_api::decl_runtime_apis;

decl_runtime_apis! {
    /// API for the Risk Ratings Pallet.
    pub trait RiskRatingApi {
        /// Returns a static greeting from the pallet as a vector of bytes.
        fn say_hello() -> Vec<u8>;
    }
}
