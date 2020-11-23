use miniscript::bitcoin::hashes::hex::{FromHex, ToHex};
use miniscript::bitcoin::{Network, Script};
use miniscript::{descriptor::Descriptor, Miniscript, Segwitv0};
use serde::Serialize;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

use crate::{parse, Evaluate, Policy, Result, Scope, Value};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize)]
pub struct JsResult {
    policy: String,
    miniscript: String,
    script_hex: String,
    script_asm: String,
    descriptor: String,
    child_desc: String,
    spk: String,
    address: String,
}

#[wasm_bindgen(js_name = compile)]
pub fn js_compile(
    code: &str,
    desc_type: &str,
    network: &str,
    child_code: u32,
) -> std::result::Result<JsValue, JsValue> {
    let network = Network::from_str(network).map_err(|e| e.to_string())?;

    let policy = compile(code).map_err(|e| e.to_string())?;
    let miniscript: Miniscript<_, Segwitv0> = policy.compile().map_err(|e| e.to_string())?;
    let script = miniscript.encode();

    let descriptor = match desc_type {
        "wsh" => Descriptor::Wsh(miniscript.clone()),
        "shwsh" => Descriptor::ShWsh(miniscript.clone()),
        _ => bail!("Unsupported descriptor type"),
    };

    let child_desc = descriptor.derive(child_code.into());

    let spk = child_desc.script_pubkey();
    let address = child_desc.address(network).unwrap();

    Ok(JsValue::from_serde(&JsResult {
        policy: policy.to_string(),
        miniscript: miniscript.to_string(),
        script_hex: script.to_hex(),
        script_asm: get_script_asm(&script),
        descriptor: descriptor.to_string(),
        child_desc: child_desc.to_string(),
        spk: spk.to_string(),
        address: address.to_string(),
    })
    .unwrap())
}

fn compile(code: &str) -> Result<Policy> {
    parse(code)?.eval(&DEMO_SCOPE)?.into_policy()
}

fn get_script_asm(script: &Script) -> String {
    let s = format!("{:?}", script);
    s[7..s.len() - 1].into()
}

lazy_static! {
    // Provide some built-in example pubkeys and hashes in the web demo env
    static ref DEMO_SCOPE: Scope<'static> = {
        let mut scope = Scope::root();
        let mut add_key = |name, key: &str| {
            scope
                .set(name, Value::PubKey(key.parse().unwrap()))
                .unwrap();
        };
        add_key(
            "A",
            "029ffbe722b147f3035c87cb1c60b9a5947dd49c774cc31e94773478711a929ac0",
        );
        add_key(
            "B",
            "025f05815e3a1a8a83bfbb03ce016c9a2ee31066b98f567f6227df1d76ec4bd143",
        );
        add_key(
            "C",
            "025625f41e4a065efc06d5019cbbd56fe8c07595af1231e7cbc03fafb87ebb71ec",
        );
        add_key(
            "D",
            "02a27c8b850a00f67da3499b60562673dcf5fdfb82b7e17652a7ac54416812aefd",
        );
        add_key(
            "E",
            "03e618ec5f384d6e19ca9ebdb8e2119e5bef978285076828ce054e55c4daf473e2",
        );
        add_key(
            "F",
            "03deae92101c790b12653231439f27b8897264125ecb2f46f48278603102573165",
        );
        add_key(
            "G",
            "033841045a531e1adf9910a6ec279589a90b3b8a904ee64ffd692bd08a8996c1aa",
        );
        add_key(
            "I",
            "02aebf2d10b040eb936a6f02f44ee82f8b34f5c1ccb20ff3949c2b28206b7c1068",
        );
        add_key(
            "J",
            "03d2810d442a784e93133760af5ac05e4eb72364a3257e5a5eafc618ccb15e580a",
        );
        add_key(
            "K",
            "03a81dca4cde2edf3d193e2b2446b40aa04f33dd11a4599c7fa55415fc274f0f70",
        );
        add_key(
            "L",
            "029e5de3f2391700fdb5f45aa5db40b953de8bd4a147663b1cd89aa0703a0c2fcf",
        );

        add_key(
            "user_pk",
            "03c620141755e90c86ec35fe57594e0b4b1a32f09f15bc0a43b06f9feb71c1b06c",
        );
        add_key(
            "service_pk",
            "02f8b2c15f9e301d7e46169a35088724cbcb264f678d628d615c38ee964f836245",
        );
        add_key(
            "buyer",
            "03829e91bb8d4df87fea147f98ef5d3e71c7c26204a5ed5de2d1d966938d017ac2",
        );
        add_key(
            "seller",
            "0215152236dd9f518dd2bba50487857b98bdb4778c3618780a25a0cbc660092185",
        );
        add_key(
            "arbiter",
            "0203bc5458e2b77b5f5a68a738a57bee0271a27e603100c4110533bf8811c19e2e",
        );

        let mut add_hash = |name, hash: &str| {
            scope
                .set(name, Value::Hash(Vec::from_hex(hash).unwrap()))
                .unwrap();
        };
        add_hash(
            "H",
            "01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b",
        );
        add_hash("H1", "4355a46b19d348dc2f57c046f8ef63d4538ebb93");
        add_hash("H2", "53c234e5e8472b6ac51c1ae1cab3fe06fad053be");
        scope
    };
}
