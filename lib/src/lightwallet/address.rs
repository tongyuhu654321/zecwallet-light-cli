//! Structs for handling supported address types.

use pairing::bls12_381::Bls12;
use zcash_primitives::primitives::PaymentAddress;
use zcash_client_backend::encoding::{decode_payment_address};
use zcash_primitives::legacy::TransparentAddress;
use base58::{FromBase58, FromBase58Error};

pub fn decode_transparent_address(
    pubkey_version: &[u8],
    script_version: &[u8],
    s: &str,
) -> Result<Option<TransparentAddress>, FromBase58Error> {
    //let decoded = bs58::decode(s).into_vec()?;
    let decoded_with_check = s.from_base58()?;
    // Remove the last 4 bytes, which is the checksum
    let decoded = decoded_with_check[..decoded_with_check.len()-4].to_vec();
    
    if &decoded[..pubkey_version.len()] == pubkey_version {
        println!("Decoded pubkey, len = {}", decoded.len());
        if decoded.len() == pubkey_version.len() + 20 {
            let mut data = [0; 20];
            data.copy_from_slice(&decoded[pubkey_version.len()..]);
            Ok(Some(TransparentAddress::PublicKey(data)))
        } else {
            Ok(None)
        }
    } else if &decoded[..script_version.len()] == script_version {
        println!("Decoded script, len = {}", decoded.len());
        if decoded.len() == script_version.len() + 20 {
            let mut data = [0; 20];
            data.copy_from_slice(&decoded[script_version.len()..]);
            Ok(Some(TransparentAddress::Script(data)))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

/// An address that funds can be sent to.
pub enum RecipientAddress {
    Shielded(PaymentAddress<Bls12>),
    Transparent(TransparentAddress),
}

impl From<PaymentAddress<Bls12>> for RecipientAddress {
    fn from(addr: PaymentAddress<Bls12>) -> Self {
        RecipientAddress::Shielded(addr)
    }
}

impl From<TransparentAddress> for RecipientAddress {
    fn from(addr: TransparentAddress) -> Self {
        RecipientAddress::Transparent(addr)
    }
}

impl RecipientAddress {
    pub fn from_str(s: &str, hrp_sapling_address: &str, b58_pubkey_address: [u8; 2], b58_script_address: [u8; 2]) -> Option<Self> {
        // Try to match a sapling z address 
        if let Some(pa) = match decode_payment_address(hrp_sapling_address, s) {
                                Ok(ret) => ret,
                                Err(_)  => None
                            } 
        {
            Some(RecipientAddress::Shielded(pa))    // Matched a shielded address
        } else if let Some(addr) = match decode_transparent_address(
                                            &b58_pubkey_address, &b58_script_address, s) {
                                        Ok(ret) => ret,
                                        Err(_)  => None
                                    } 
        {
            Some(RecipientAddress::Transparent(addr))   // Matched a transparent address
        } else {
            None    // Didn't match anything
        }
    }
}
