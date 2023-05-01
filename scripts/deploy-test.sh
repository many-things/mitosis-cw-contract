#!/bin/sh

OWNER=${GOV:-"osmo1c5etmwyca4hyf0nswjtkarxm5n2hqqrjjv07vs"}
BEAKER=${BEAKER:-"beaker"}
DAEMON=${DAEMON:-"osmosisd"}
MNEMONIC=${SIGNER_MNEMONIC:-"derive miracle banana bright timber energy noodle half they jealous gossip flight keen reject kid goose together collect lecture sentence ball solid fan stereo"}
DENOM=${DENOM:-"uusdc"}
LP_DENOM=${LP_DENOM:-"ulpusdc"}

SIGNER="test-deployer"
beaker key set "$SIGNER" "$MNEMONIC" -y
(echo "y"; echo "$MNEMONIC") | $DAEMON keys add --recover "$SIGNER"

TOKENFACTORY_FEE=$(
    $DAEMON query tokenfactory params \
        --output=json \
        --node=$NODE | \
    jq -r '.params.denom_creation_fee[0] | .amount + .denom'
)
echo $TOKENFACTORY_FEE

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
    denom_manager

STATES=$([ "$NETWORK" = "local" ] && echo "state.local.json" || echo "state.json")
DENOM_MGR_ADDR=$(cat $(pwd)/.beaker/$STATES | jq -r '.'$NETWORK'["denom_manager"].addresses.default')
echo "DMGR ADDR: $DENOM_MGR_ADDR"

check "$DENOM_MGR_ADDR" "DENOM_MGR_ADDR"

echo "================ Deploying liquidity manager contracts ================"
LM_INIT_MSG=$(
    cat $(pwd)/scripts/$NETWORK/liquidity_manager.json | \
    jq -c '.denom = "'$DENOM'"' | \
    jq -c '.lp_denom = "'$LP_DENOM'"'
)
beaker wasm deploy \
    --raw $LM_INIT_MSG \
    --network $NETWORK \
    --admin "signer" \
    --funds $TOKENFACTORY_FEE \
    $SIGNER_FLAG \
    $OPTIMIZE_FLAG \
    liquidity-manager
LMGR_ADDR=$(cat $(pwd)/.beaker/state.json | jq -r '.'$NETWORK'["liquidity-manager"].addresses.default')
echo "LMGR ADDR: $LMGR_ADDR"

check "$LMGR_ADDR" "LMGR_ADDR"


echo "================ Deploying gateway contracts ================"
GW_INIT_MSG=$(
    cat $(pwd)/scripts/$NETWORK/gateway.json | \
    jq -c '.denom_manager = "'$DENOM_MGR_ADDR'"' | \
    jq -c '.liquidity_manager = "'$LMGR_ADDR'"'
)
beaker wasm deploy \
    --raw $GW_INIT_MSG \
    --network $NETWORK \
    --admin "signer" \
    $SIGNER_FLAG \
    $OPTIMIZE_FLAG \
    mitosis-gateway
