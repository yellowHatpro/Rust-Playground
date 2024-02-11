use bdk::bitcoin::Network;
use bdk::database::MemoryDatabase;
use bdk::keys::{DerivableKey, GeneratableKey, GeneratedKey, ExtendedKey, bip39::{Mnemonic, WordCount, Language}};
use bdk::{KeychainKind, miniscript, Wallet};
use bdk::template::Bip84;

fn create_wallet() -> (Wallet<MemoryDatabase>,String) {
    let network = Network::Testnet;
    let mnemonic: GeneratedKey<_, miniscript::Segwitv0> = Mnemonic::generate((WordCount::Words12, Language::English)).unwrap();
    let mnemonic_words = mnemonic.to_string();
    let mnemonic = Mnemonic::parse(&mnemonic_words).unwrap();
    let xkey: ExtendedKey = mnemonic.into_extended_key().unwrap();
    let xprv = xkey.into_xprv(network).unwrap();
    let wallet = Wallet::new(
        Bip84(xprv, KeychainKind::External),
        Some(Bip84(xprv, KeychainKind::Internal)),
        network,
        MemoryDatabase::default(),
    ).unwrap();
    return (wallet, mnemonic_words)
}

fn main() {
    let (sender, mnemonic_words1) = create_wallet();
    let (receiver, mnemonic_words2) = create_wallet();
    println!("mnemonic of sender: {}\n\nrecv desc (pub key): {:#?}\n\nchng desc (pub key): {:#?}",
             mnemonic_words1,
             sender.get_descriptor_for_keychain(KeychainKind::External).to_string(),
             sender.get_descriptor_for_keychain(KeychainKind::Internal).to_string());
    println!("mnemonic of receiver: {}\n\nrecv desc (pub key): {:#?}\n\nchng desc (pub key): {:#?}",
             mnemonic_words2,
             receiver.get_descriptor_for_keychain(KeychainKind::External).to_string(),
             receiver.get_descriptor_for_keychain(KeychainKind::Internal).to_string());
}
