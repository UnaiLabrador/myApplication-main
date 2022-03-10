// Copyright (c) The Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use forge::{forge_main, ForgeConfig, LocalFactory, Options, Result};
use smoke_test::aptos::{AccountCreation, GasCheck, MintTransfer, ModulePublish};

fn main() -> Result<()> {
    let tests = ForgeConfig::default()
        .with_aptos_tests(&[&AccountCreation, &MintTransfer, &GasCheck, &ModulePublish])
        .with_genesis_modules_bytes(aptos_framework_releases::current_module_blobs().to_vec());

    let options = Options::from_args();
    forge_main(tests, LocalFactory::from_workspace()?, &options)
}
