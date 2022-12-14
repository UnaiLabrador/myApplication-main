// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

//! This module provides common utilities for the DB pruner.

use crate::{
    pruner::{
        db_pruner::DBPruner, ledger_store::ledger_store_pruner::LedgerPruner,
        state_store::StateStorePruner,
    },
    EventStore, LedgerStore, TransactionStore,
};
use aptos_config::config::StoragePrunerConfig;
use aptos_infallible::Mutex;
use schemadb::DB;
use std::sync::Arc;

/// Utility functions to instantiate pruners.
pub fn create_state_pruner(
    state_merkle_db: Arc<DB>,
    storage_pruner_config: StoragePrunerConfig,
) -> Option<Mutex<Arc<dyn DBPruner + Send + Sync>>> {
    if storage_pruner_config.state_store_prune_window.is_some() {
        Some(Mutex::new(Arc::new(StateStorePruner::new(Arc::clone(
            &state_merkle_db,
        )))))
    } else {
        None
    }
}

pub fn create_ledger_pruner(
    ledger_db: Arc<DB>,
    storage_pruner_config: StoragePrunerConfig,
) -> Option<Mutex<Arc<dyn DBPruner + Send + Sync>>> {
    if storage_pruner_config.ledger_prune_window.is_some() {
        Some(Mutex::new(Arc::new(LedgerPruner::new(
            Arc::clone(&ledger_db),
            Arc::new(TransactionStore::new(Arc::clone(&ledger_db))),
            Arc::new(EventStore::new(Arc::clone(&ledger_db))),
            Arc::new(LedgerStore::new(Arc::clone(&ledger_db))),
        ))))
    } else {
        None
    }
}
