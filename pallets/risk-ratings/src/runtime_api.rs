// use sp_api::decl_runtime_apis;
// use sp_std::vec::Vec;

// decl_runtime_apis! {
//     /// Runtime API for Risk Ratings
//     ///
//     /// This API exposes functionality to query risk ratings data
//     /// via RPC using state_call.
//     #[api_version(1)]
//     pub trait RiskRatingsApi {
//         /// Returns a hello message from the Risk Ratings pallet.
//         ///
//         /// Note: This returns Vec<u8> (not &str) for compatibility with the
//         /// WASM runtime boundary. The runtime implementation converts the
//         /// internal &'static str to Vec<u8>.
//         ///
//         /// This function can be called via RPC using state_call:
//         /// ```
//         /// api.rpc.state.call('RiskRatingsApi_say_hello', '')
//         /// ```
//         fn say_hello() -> Vec<u8>;
//     }
// } 