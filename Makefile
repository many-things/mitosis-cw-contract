deploy-testnet:
	NETWORK="testnet" \
	CHAIN_ID="osmo-test-4" \
	NODE="https://rpc-test.osmosis.zone:443" \
	RUST_BACKTRACE=1 \
	./scripts/deploy-test.sh

schema:
	ls ./contracts | xargs -n 1 -t beaker wasm ts-gen
	