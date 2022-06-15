#!/bin/bash

#
# Runs an automated genesis ceremony for validators spun up by the aptos-node helm chart
#
# Expect the following environment variables to be set before execution:
# NUM_VALIDATORS
# ERA
# WORKSPACE: default /tmp
# USERNAME_PREFIX: default aptos-node
# VALIDATOR_HOST_SUFFIX: default validator-lb
# FULLNODE_HOST_SUFFIX: default fullnode-lb
# 

WORKSPACE=${WORKSPACE:-/tmp}
USERNAME_PREFIX=${USERNAME_PREFIX:-aptos-node}
VALIDATOR_HOST_SUFFIX=${VALIDATOR_HOST_SUFFIX:-validator-lb}
FULLNODE_HOST_SUFFIX=${FULLNODE_HOST_SUFFIX:-fullnode-lb}

if [[ -z ${ERA} ]] || [[ -z ${NUM_VALIDATORS} ]]; then
    echo "ERA (${ERA:-null}) and NUM_VALIDATORS (${NUM_VALIDATORS:-null}) must be set"
    exit 1
fi

echo "NUM_VALIDATORS=${NUM_VALIDATORS}"
echo "ERA=${ERA}"
echo "WORKSPACE=${WORKSPACE}"
echo "USERNAME_PREFIX=${USERNAME_PREFIX}"
echo "VALIDATOR_HOST_SUFFIX=${VALIDATOR_HOST_SUFFIX}"
echo "FULLNODE_HOST_SUFFIX=${FULLNODE_HOST_SUFFIX}"

# generate all validator configurations
for i in $(seq 0 $(($NUM_VALIDATORS-1))); do
username="${USERNAME_PREFIX}-${i}"
user_dir="${WORKSPACE}/${username}"
mkdir $user_dir
aptos genesis generate-keys --output-dir $user_dir
aptos genesis set-validator-configuration --keys-dir $user_dir --local-repository-dir $WORKSPACE \
    --username $username \
    --validator-host "${username}-${VALIDATOR_HOST_SUFFIX}:6180" \
    --full-node-host "${username}-${FULLNODE_HOST_SUFFIX}:6182"
done

# get the framework
# this is the directory the aptos-framework is located in the aptoslabs/init docker image
FRAMEWORK_DIR="/aptos-framework/move/modules"
cp -R $FRAMEWORK_DIR ${WORKSPACE}/framework

# run genesis
aptos genesis generate-genesis --local-repository-dir ${WORKSPACE} --output-dir ${WORKSPACE}

# delete all fullnode storage except for those from this era
kubectl get pvc -o name | grep /fn- | grep -v "e${ERA}-" | xargs -r kubectl delete
# delete all genesis secrets except for those from this era
kubectl get secret -o name | grep "genesis-e" | grep -v "e${ERA}-" | xargs -r kubectl delete

# create genesis secrets for validators to startup
for i in $(seq 0 $(($NUM_VALIDATORS-1))); do
username="${USERNAME_PREFIX}-${i}"
user_dir="${WORKSPACE}/${username}"
kubectl create secret generic "${username}-genesis-e${ERA}" \
    --from-file=genesis.blob=${WORKSPACE}/genesis.blob \
    --from-file=waypoint.txt=${WORKSPACE}/waypoint.txt \
    --from-file=validator-identity.yaml=${user_dir}/validator-identity.yaml \
    --from-file=validator-full-node-identity.yaml=${user_dir}/validator-full-node-identity.yaml
done
