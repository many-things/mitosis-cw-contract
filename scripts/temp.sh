
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
    $SIGNER_FLAG \
    $OPTIMIZE_FLAG \
    mitosis-liquidity-manager
LMGR_ADDR=$(cat $(pwd)/.beaker/$STATES | jq -r '.'$NETWORK'["mitosis-liquidity-manager"].addresses.default')
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

