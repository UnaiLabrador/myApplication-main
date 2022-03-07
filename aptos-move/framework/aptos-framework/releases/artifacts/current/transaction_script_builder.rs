// Copyright (c) The Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

// This file was generated. Do not modify!
//
// To update this code, run: `cargo run --release -p framework`.

//! Conversion library between a structured representation of a Move script call (`ScriptCall`) and the
//! standard BCS-compatible representation used in Diem transactions (`Script`).
//!
//! This code was generated by compiling known Script interfaces ("ABIs") with the tool `transaction-builder-generator`.

#![allow(clippy::unnecessary_wraps)]
#![allow(unused_imports)]
use aptos_types::{
    account_address::AccountAddress,
    transaction::{Script, ScriptFunction, TransactionArgument, TransactionPayload, VecBytes},
};
use move_core_types::{
    ident_str,
    language_storage::{ModuleId, TypeTag},
};
use std::collections::BTreeMap as Map;

type Bytes = Vec<u8>;

/// Structured representation of a call into a known Move script function.
/// ```ignore
/// impl ScriptFunctionCall {
///     pub fn encode(self) -> TransactionPayload { .. }
///     pub fn decode(&TransactionPayload) -> Option<ScriptFunctionCall> { .. }
/// }
/// ```
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "fuzzing", derive(proptest_derive::Arbitrary))]
#[cfg_attr(feature = "fuzzing", proptest(no_params))]
pub enum ScriptFunctionCall {
    CreateAccount {
        new_account_address: AccountAddress,
        auth_key_prefix: Bytes,
    },

    Mint {
        addr: AccountAddress,
        amount: u64,
    },

    Transfer {
        to: AccountAddress,
        amount: u64,
    },
}

impl ScriptFunctionCall {
    /// Build a Diem `TransactionPayload` from a structured object `ScriptFunctionCall`.
    pub fn encode(self) -> TransactionPayload {
        use ScriptFunctionCall::*;
        match self {
            CreateAccount {
                new_account_address,
                auth_key_prefix,
            } => encode_create_account_script_function(new_account_address, auth_key_prefix),
            Mint { addr, amount } => encode_mint_script_function(addr, amount),
            Transfer { to, amount } => encode_transfer_script_function(to, amount),
        }
    }

    /// Try to recognize a Diem `TransactionPayload` and convert it into a structured object `ScriptFunctionCall`.
    pub fn decode(payload: &TransactionPayload) -> Option<ScriptFunctionCall> {
        if let TransactionPayload::ScriptFunction(script) = payload {
            match SCRIPT_FUNCTION_DECODER_MAP.get(&format!(
                "{}{}",
                script.module().name(),
                script.function()
            )) {
                Some(decoder) => decoder(payload),
                None => None,
            }
        } else {
            None
        }
    }
}

pub fn encode_create_account_script_function(
    new_account_address: AccountAddress,
    auth_key_prefix: Vec<u8>,
) -> TransactionPayload {
    TransactionPayload::ScriptFunction(ScriptFunction::new(
        ModuleId::new(
            AccountAddress::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
            ident_str!("BasicScripts").to_owned(),
        ),
        ident_str!("create_account").to_owned(),
        vec![],
        vec![
            bcs::to_bytes(&new_account_address).unwrap(),
            bcs::to_bytes(&auth_key_prefix).unwrap(),
        ],
    ))
}

pub fn encode_mint_script_function(addr: AccountAddress, amount: u64) -> TransactionPayload {
    TransactionPayload::ScriptFunction(ScriptFunction::new(
        ModuleId::new(
            AccountAddress::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
            ident_str!("BasicScripts").to_owned(),
        ),
        ident_str!("mint").to_owned(),
        vec![],
        vec![
            bcs::to_bytes(&addr).unwrap(),
            bcs::to_bytes(&amount).unwrap(),
        ],
    ))
}

pub fn encode_transfer_script_function(to: AccountAddress, amount: u64) -> TransactionPayload {
    TransactionPayload::ScriptFunction(ScriptFunction::new(
        ModuleId::new(
            AccountAddress::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
            ident_str!("BasicScripts").to_owned(),
        ),
        ident_str!("transfer").to_owned(),
        vec![],
        vec![bcs::to_bytes(&to).unwrap(), bcs::to_bytes(&amount).unwrap()],
    ))
}

fn decode_create_account_script_function(
    payload: &TransactionPayload,
) -> Option<ScriptFunctionCall> {
    if let TransactionPayload::ScriptFunction(script) = payload {
        Some(ScriptFunctionCall::CreateAccount {
            new_account_address: bcs::from_bytes(script.args().get(0)?).ok()?,
            auth_key_prefix: bcs::from_bytes(script.args().get(1)?).ok()?,
        })
    } else {
        None
    }
}

fn decode_mint_script_function(payload: &TransactionPayload) -> Option<ScriptFunctionCall> {
    if let TransactionPayload::ScriptFunction(script) = payload {
        Some(ScriptFunctionCall::Mint {
            addr: bcs::from_bytes(script.args().get(0)?).ok()?,
            amount: bcs::from_bytes(script.args().get(1)?).ok()?,
        })
    } else {
        None
    }
}

fn decode_transfer_script_function(payload: &TransactionPayload) -> Option<ScriptFunctionCall> {
    if let TransactionPayload::ScriptFunction(script) = payload {
        Some(ScriptFunctionCall::Transfer {
            to: bcs::from_bytes(script.args().get(0)?).ok()?,
            amount: bcs::from_bytes(script.args().get(1)?).ok()?,
        })
    } else {
        None
    }
}

type ScriptFunctionDecoderMap = std::collections::HashMap<
    String,
    Box<
        dyn Fn(&TransactionPayload) -> Option<ScriptFunctionCall>
            + std::marker::Sync
            + std::marker::Send,
    >,
>;

static SCRIPT_FUNCTION_DECODER_MAP: once_cell::sync::Lazy<ScriptFunctionDecoderMap> =
    once_cell::sync::Lazy::new(|| {
        let mut map: ScriptFunctionDecoderMap = std::collections::HashMap::new();
        map.insert(
            "BasicScriptscreate_account".to_string(),
            Box::new(decode_create_account_script_function),
        );
        map.insert(
            "BasicScriptsmint".to_string(),
            Box::new(decode_mint_script_function),
        );
        map.insert(
            "BasicScriptstransfer".to_string(),
            Box::new(decode_transfer_script_function),
        );
        map
    });
