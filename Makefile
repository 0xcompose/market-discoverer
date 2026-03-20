.PHONY: all
all: 
	cargo build --release
	./target/release/market-discoverer ./config/ethereum_list.toml
	./target/release/market-discoverer ./config/geckoterminal.toml
	./target/release/market-discoverer ./config/stargate_chains.toml
	./target/release/market-discoverer ./config/coingecko.toml
	./target/release/market-discoverer ./config/layerzero.toml

.PHONY: run
run:
	./target/release/market-discoverer ethereum-list
	./target/release/market-discoverer geckoterminal
	./target/release/market-discoverer stargate-api
	./target/release/market-discoverer coingecko
	./target/release/market-discoverer layer-zero
