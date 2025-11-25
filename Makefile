.PHONY: all
all: 
	cargo build --release
	./target/release/market-discoverer ./config/ethereum_list.toml
	./target/release/market-discoverer ./config/geckoterminal.toml
	./target/release/market-discoverer ./config/stargate_chains.toml
	./target/release/market-discoverer ./config/coingecko.toml
	./target/release/market-discoverer ./config/layerzero.toml