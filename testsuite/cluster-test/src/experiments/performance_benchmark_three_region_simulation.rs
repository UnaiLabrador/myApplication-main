// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{
    cluster::Cluster,
    effects::{self, network_delay},
    experiments::{Context, Experiment, ExperimentParam},
    tx_emitter::EmitJobRequest,
};
use anyhow::Result;
use async_trait::async_trait;
use libra_logger::info;
use std::{
    fmt::{Display, Error, Formatter},
    time::Duration,
};
use structopt::StructOpt;

pub struct PerformanceBenchmarkThreeRegionSimulation {
    cluster: Cluster,
}

#[derive(StructOpt, Debug)]
pub struct PerformanceBenchmarkThreeRegionSimulationParams {}

impl ExperimentParam for PerformanceBenchmarkThreeRegionSimulationParams {
    type E = PerformanceBenchmarkThreeRegionSimulation;
    fn build(self, cluster: &Cluster) -> Self::E {
        Self::E {
            cluster: cluster.clone(),
        }
    }
}

#[async_trait]
impl Experiment for PerformanceBenchmarkThreeRegionSimulation {
    async fn run(&mut self, context: &mut Context<'_>) -> anyhow::Result<()> {
        let num_nodes = self.cluster.validator_instances().len();
        let split_country_num = ((num_nodes as f64) * 0.8) as usize;
        let split_region_num = split_country_num / 2;
        let (us, euro) = self.cluster.split_n_validators_random(split_country_num);
        let (us_west, us_east) = us.split_n_validators_random(split_region_num);
        let mut effects = network_delay::three_region_simulation_effects(
            (
                us_west.validator_instances().to_vec(),
                us_east.validator_instances().to_vec(),
                euro.validator_instances().to_vec(),
            ),
            (
                Duration::from_millis(60), // us_east<->eu one way delay
                Duration::from_millis(95), // us_west<->eu one way delay
                Duration::from_millis(40), // us_west<->us_east one way delay
            ),
        );

        effects::activate_all(&mut effects).await?;

        let window = Duration::from_secs(240);
        let emit_job_request = if context.emit_to_validator {
            EmitJobRequest::for_instances(
                context.cluster.validator_instances().to_vec(),
                context.global_emit_job_request,
            )
        } else {
            EmitJobRequest::for_instances(
                context.cluster.fullnode_instances().to_vec(),
                context.global_emit_job_request,
            )
        };
        let stats = context
            .tx_emitter
            .emit_txn_for(window, emit_job_request)
            .await?;
        let avg_tps = stats.committed / window.as_secs();
        let avg_latency_client = stats.latency / stats.committed;
        let p99_latency = stats.latency_buckets.percentile(99, 100);
        let submitted_txn = stats.submitted;
        let expired_txn = stats.expired;
        info!(
            "Tx status from client side: txn {}, avg latency {}",
            stats.committed as u64, avg_latency_client
        );

        effects::deactivate_all(&mut effects).await?;
        context
            .report
            .report_metric(&self, "submitted_txn", submitted_txn as f64);
        context
            .report
            .report_metric(&self, "expired_txn", expired_txn as f64);
        context
            .report
            .report_metric(&self, "avg_tps", avg_tps as f64);
        context
            .report
            .report_metric(&self, "avg_latency", avg_latency_client as f64);
        context
            .report
            .report_metric(&self, "p99_latency", p99_latency as f64);
        let expired_text = if expired_txn == 0 {
            "no expired txns".to_string()
        } else {
            format!("(!) expired {} out of {} txns", expired_txn, submitted_txn)
        };
        context.report.report_text(format!(
            "{} : {:.0} TPS, {:.1} ms latency, {:.1} ms p99 latency, {}",
            self, avg_tps, avg_latency_client, p99_latency, expired_text
        ));
        Ok(())
    }

    fn deadline(&self) -> Duration {
        Duration::from_secs(600)
    }
}

impl Display for PerformanceBenchmarkThreeRegionSimulation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "3 Region Simulation")
    }
}
