// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

pub mod common;
pub mod move_tool;
pub mod op;
pub mod list;

use crate::common::types::{CliResult, Error};
use clap::Parser;

/// CLI tool for interacting with the Aptos blockchain and nodes
///
#[derive(Parser)]
#[clap(name = "aptos", author, version, propagate_version = true)]
pub enum Tool {
    List(list::ListResources),
    #[clap(subcommand)]
    Move(move_tool::MoveTool),
    #[clap(subcommand)]
    Op(op::OpTool),
}

impl Tool {
    pub async fn execute(self) -> CliResult {
        match self {
            Tool::List(list_tool) => list_tool.execute().await,
            Tool::Move(tool) => tool.execute().await,
            Tool::Op(op_tool) => op_tool.execute().await,
        }
    }
}
