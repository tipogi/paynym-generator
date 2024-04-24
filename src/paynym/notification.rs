use bitcoin::{base58, hex::DisplayHex};
use crate::error::PaynymError;

use super::Paynym;



static BOB_PAYNYM: &str = "PM8TJYp8zHvhimVNRjUcEuULfmvmUML6YTbTSnBuU69MYy93AzsXELFLaVjpxc5mxDex7R8ttgtL1tGAt2TshZAoFeB5zn4c9nRo4oZpmuyuo4FTpUrd";
pub struct PNotification {}

impl PNotification {
    pub fn create_notification_tx() -> Result<(), PaynymError> {
        let decoded_paynym = base58::decode(BOB_PAYNYM).unwrap();
        let hex_paynym = decoded_paynym.to_lower_hex_string();
        let bob_pub_key = &hex_paynym[3..36];
        let bob_chaincode = &hex_paynym[36..58];
        if !PNotification::correct_checksum(&hex_paynym) { return Err(PaynymError::Checksum) }

        Ok(())
    }

    fn correct_checksum(paynym_with_checksum: &str) -> bool {
        let paynym_code = &paynym_with_checksum[..paynym_with_checksum.len() - 8];
        let checksum = Paynym::get_checksum(paynym_code);
        let far_party_paynym_checksum = &paynym_with_checksum[paynym_with_checksum.len() - 8 ..];
        checksum == far_party_paynym_checksum
    }
}