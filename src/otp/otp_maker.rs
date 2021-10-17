use std::convert::TryInto;
use std::fmt::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use data_encoding::{BASE32_NOPAD, DecodeError};
use totp_lite::{Sha1, Sha256, Sha512, totp_custom};
use hmac::{Hmac, Mac, NewMac};
use hmac::crypto_mac::Output;
use hmac::digest::{BlockInput, FixedOutputDirty, Reset, Update};

pub fn totp(secret: &str, algorithm: &str, digits: u32) -> Result<String,String>{
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    generate_totp(secret,algorithm,digits,time,30,0)
}

fn generate_totp(secret: &str, algorithm: &str, digits: u32, time: u64,time_step: u64, skew: i64) -> Result<String,String> {
    hotp(secret,algorithm,digits,((time as i64 + skew) as u64) / time_step)
}

pub fn hotp(secret: &str, algorithm: &str, digits: u32, counter: u64) -> Result<String,String>{
    return match algorithm {
        "SHA256" => generate_hotp::<Sha256>(secret,digits,counter),
        "SHA512" => generate_hotp::<Sha512>(secret,digits,counter),
        _ => generate_hotp::<Sha1>(secret,digits,counter),
    }
}

fn generate_hotp<A>(secret: &str, digits: u32, counter: u64) -> Result<String,String>
where A: Update + BlockInput + Reset + FixedOutputDirty + Clone + Default {
    // decode the base32 secret
    let secret_decoded = match BASE32_NOPAD.decode(secret.as_bytes()) {
        Ok(result) => result,
        Err(e) => return Err(format!("{:?}",e)),
    };
    // calculate HMAC from secret bytes and counter
    let mut hmac: Hmac<A> = Hmac::new_from_slice(secret_decoded.as_slice()).expect("Failed to derive HMAC");
    hmac.update(&counter.to_be_bytes());
    let hash = hmac.finalize().into_bytes();

    // calculate offset
    let offset: usize = match hash.last() {
        Some(result) => *result & 0xf,
        None => return Err(String::from("Invalid digest")),
    } as usize;

    // calculate code
    let code_bytes: [u8; 4] = match hash[offset..offset+4].try_into() {
        Ok(x) => x,
        Err(_) => return Err(String::from("Invalid digest"))
    };
    let code: String = ((u32::from_be_bytes(code_bytes) & 0x7fffffff) % 10_u32.pow(digits)).to_string();
    Ok("0".repeat(digits as usize - code.chars().count()) + code.as_str())
}

#[cfg(test)]
mod tests {
    use sha1::Sha1;
    use crate::otp::otp_maker::{generate_totp, generate_hotp};

    #[test]
    fn test_totp() {
        assert_eq!(generate_totp("BASE32SECRET3232", "SHA1", 6, 0,30, 0).unwrap(), "260182");
    }

    #[test]
    fn test_hotp(){
        assert_eq!(generate_hotp::<Sha1>("BASE32SECRET3232", 6,0).unwrap(), "260182");
        assert_eq!(generate_hotp::<Sha1>("BASE32SECRET3232", 6,1).unwrap(), "055283");
    }
}
