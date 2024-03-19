mod r#impl;
mod r#type;
mod api_type;
mod api_impl;
mod r#trait;
pub mod error;
pub mod api;

pub use api_type::*;
pub use r#type::*;
pub use r#trait::*;

pub use r#impl::nonce;


#[cfg(test)]
mod tests {
    use hex::ToHex;

    use crate::r#impl::signature;

    use super::*;

    #[test]
    fn test_sig() {
        let nonce = "cdw8wkb8bactehbrijem2keaif87szmd";
        let method = "post";
        let uri = "/auth/sign-in";
        let time = 1710591930;
        let t_sig = "dc48be9d4f933999ef84555f08323afb98d5337869001c1de5eb147969156c03";
        let sig = signature(uri, time, nonce, method).unwrap().encode_hex::<String>();
        println!("t_sig: {}", t_sig);
        println!("sig: {}", sig);
    }
}
