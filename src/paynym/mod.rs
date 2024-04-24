pub mod notification;

use std::str::FromStr;
use bip39::Mnemonic;
use bitcoin::bip32::{DerivationPath, Xpriv, Xpub};
use bitcoin::key::Secp256k1;
use bitcoin::secp256k1::ffi::types::AlignedType;
use bitcoin::hex::{Case, DisplayHex};
use bitcoin::{base58, Network};
use sha2::{Sha256, Digest};

static PREFIX: &str = "47";
static VERSION: &str = "01";
static BIT_FIELD: &str = "00";
static RESERVED: &str = "00000000000000000000000000";

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Paynym {
    xpriv: Xpriv,
    xpub: Xpub,
    code: String
}

impl Paynym {

    fn create_seed(mnemonic: &str) -> [u8; 64] {
        // Convert the mnemonic in array of numbers. That numbers match the BIP39 wordlist
        let mnemonic = Mnemonic::from_str(mnemonic).unwrap();
        println!("Mnemonic, word list: {:?}", mnemonic);
        // Missing in the array the last bits which are the checksum. We get after sha256 the entropy bits
        // 12 WORDS: The first 4 bits of checksum. 
        // Each word is 11 bits. Convert each decimal of the entropy array and make groups of 11
        println!("Entropy: {:?}", mnemonic.to_entropy());
        
        let seed = mnemonic.to_seed_normalized("");
        let hex_seed = seed.to_hex_string(Case::Lower);
        println!("HEX format seed: {:?}", hex_seed);

        seed
    }

    fn create_code(xpub: Xpub) -> String {
        let public_key = xpub.public_key.to_string();
        let chain_code = xpub.chain_code.to_string();
        let code = format!("{}{}{}{}{}{}", PREFIX, VERSION, BIT_FIELD, public_key, chain_code, RESERVED);
        let checksum = Paynym::get_checksum(&code);
        
        let code_with_checksum = format!("{}{}", code, checksum);
        
        let bytes = hex::decode(code_with_checksum).unwrap();
        let hex_slice: &[u8] = &bytes;
        let base58_encoded = base58::encode(hex_slice);
        println!("PAYNYM code: {:?}", base58_encoded);
        base58_encoded
    }

    // The input that we add in the sha it has to be HEX format
    fn get_checksum(paynym_code: &str) -> String {
        let hex_vector = hex::decode(String::from(paynym_code)).unwrap();
        let mut hex_slice: &[u8] = &hex_vector;
        let first_iter_hash = Sha256::digest(hex_slice);
        // Get the HEX string from the vector of the digest
        let hash_hex_format = format!("{:x}", first_iter_hash);
        
        let bytes2 = hex::decode(hash_hex_format).unwrap();
        hex_slice = &bytes2;
        let double_hash = Sha256::digest(hex_slice);
        let double_hash_hex = format!("{:x}", double_hash);
        // Get the first 8 chars of the double hash
        double_hash_hex.chars().take(8).collect()
    }

    pub fn from_mnemonic(mnemonic: &str) -> Paynym{
        let seed = Paynym::create_seed(mnemonic);

        let mut buf: Vec<AlignedType> = Vec::new();
        buf.resize(Secp256k1::preallocate_size(), AlignedType::zeroed());
        let secp = Secp256k1::preallocated_new(buf.as_mut_slice()).unwrap();

        // calculate root key from seed
        let root = Xpriv::new_master(Network::Bitcoin, &seed).unwrap();
        println!("Root key: {}", root);

        // derive child xpub
        let path = DerivationPath::from_str("m/47h/0h/0h").unwrap();
        let xpriv = root.derive_priv(&secp, &path).unwrap();
        println!("Child at {}: {}", path, xpriv);
        let xpub = Xpub::from_priv(&secp, &xpriv);
        println!("Public key at {}: {}", path, xpub);

        let byte_xpub = base58::decode(&xpub.to_string()).unwrap();
        println!("HEX xpub: {:?}", byte_xpub.to_lower_hex_string());
        let code = Paynym::create_code(xpub);

        Paynym {
            xpriv,
            xpub,
            code
        }
    }
}