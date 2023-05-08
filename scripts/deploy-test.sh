#!/bin/sh

OWNER=${GOV:-"osmo1jjx3kvnf0jk3fu2twfgt8wld9qtzfw08nyvm65"}
BEAKER=${BEAKER:-"beaker"}
DAEMON=${DAEMON:-"osmosisd"}
MNEMONIC=${SIGNER_MNEMONIC:-"tell disagree region twenty shock affair pipe universe popular eye resource pudding upper fashion south often spare must stamp zone pet double ski north"}
DENOM=${DENOM:-"factory/osmo1jjx3kvnf0jk3fu2twfgt8wld9qtzfw08nyvm65/uusdc"}
LP_DENOM=${LP_DENOM:-"ulpusdc"}
PUBLIC_KEY=${PUBLIC_KEY:-"039430c507a204703f511663612681ce253b0b2117edb85d8d8f807ea033e27be2"}

SIGNER="cw-deployer"
# beaker key set "$SIGNER" "$MNEMONIC" -y
# (echo "y"; echo "$MNEMONIC") | $DAEMON keys add --recover "$SIGNER"

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
    mitosis-denom-manager

STATES=$([ "$NETWORK" = "local" ] && echo "state.local.json" || echo "state.json")
DENOM_MGR_ADDR=$(cat $(pwd)/.beaker/state.json | jq -r '.'$NETWORK'["mitosis-denom-manager"].addresses.default')
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
    mitosis-liquidity-manager
LMGR_ADDR=$(cat $(pwd)/.beaker/state.json | jq -r '.'$NETWORK'["mitosis-liquidity-manager"].addresses.default')
echo "LMGR ADDR: $LMGR_ADDR"

echo "================ Deploying gateway contracts ================"
GW_INIT_MSG=$(
    cat $(pwd)/scripts/$NETWORK/gateway.json | \
    jq -c '.denom_manager = "'$DENOM_MGR_ADDR'"' | \
    jq -c '.liquidity_manager = "'$LMGR_ADDR'"' | \
    jq -c '.public_key = "'$PUBLIC_KEY'"'
)
echo $GW_INIT_MSG
beaker wasm deploy \
    --raw $GW_INIT_MSG \
    --network $NETWORK \
    --admin "signer" \
    $SIGNER_FLAG \
    $OPTIMIZE_FLAG \
    mitosis-gateway

GW_ADDR=$(cat $(pwd)/.beaker/state.json | jq -r '.'$NETWORK'["mitosis-gateway"].addresses.default')
echo "GW ADDR: $GW_ADDR"

