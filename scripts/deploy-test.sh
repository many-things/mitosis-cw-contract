#!/bin/sh

OWNER=${GOV:-"osmo1c5etmwyca4hyf0nswjtkarxm5n2hqqrjjv07vs"}
BEAKER=${BEAKER:-"beaker"}
DAEMON=${DAEMON:-"osmosisd"}
MNEMONIC=${SIGNER_MNEMONIC:-"derive miracle banana bright timber energy noodle half they jealous gossip flight keen reject kid goose together collect lecture sentence ball solid fan stereo"}
DENOM=${DENOM:-"uusdc"}
LP_DENOM=${LP_DENOM:-"ulpusdc"}

# SIGNER="test-deployer"
# beaker key set "$SIGNER" "$MNEMONIC" -y
# (echo "y"; echo "$MNEMONIC") | $DAEMON keys add --recover "$SIGNER"

function check {
    if [ -z "$1" ]
    then
      echo "\$$2 is not defined"
      exit 1
    fi
}

check "$NETWORK" "NETWORK"
check "$NODE" "NODE"
check "$CHAIN_ID" "CHAIN_ID"
check "$DENOM" "DENOM"
check "$LP_DENOM" "LP_DENOM"

SIGNER_FLAG="--signer-keyring=$SIGNER"
OPTIMIZE_FLAG=$([ "$NETWORK" = "local" ] && echo "--no-wasm-opt")

echo "================ Deploying denom manager contracts ================"
DENOM_INIT_MSG=$(cat $(pwd)/scripts/$NETWORK/denom_manager.json | jq -c)
beaker wasm deploy \
    --raw $DENOM_INIT_MSG \
    --network $NETWORK \
    --admin "signer" \
    $SIGNER_FLAG \ 
    $OPTIMIZE_FLAG \
    denommanager

STATES=$([ "$NETWORK" = "local" ] && echo "state.local.json" || echo "state.json")
DENOM_MGR_ADDR=$(cat $(pwd)/.beaker/$STATES | jq -r '.'$NETWORK'["mitosis-denom-manager"].addresses.default')
echo "DMGR ADDR: $DENOM_MGR_ADDR"

check "$DENOM_MGR_ADDR" "DENOM_MGR_ADDR"

