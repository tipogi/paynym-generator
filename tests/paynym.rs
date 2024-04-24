#[cfg(test)]
mod paynym {
    use paynym_generator::paynym::Paynym;
    use paynym_generator::paynym::notification::PNotification;
    use paynym_generator::environment::{EnvParams, PaynymEnv};

    #[test]
    fn create_paynym_code() {
        let mnemonic = PaynymEnv::initialise()
            .unwrap()
            .get(EnvParams::Mnemonic);

        Paynym::from_mnemonic(&mnemonic);
    }

    #[test]
    fn create_notification_transaction() {
        let r = PNotification::create_notification_tx().unwrap();
    }
}