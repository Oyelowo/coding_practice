build:
	cargo build-bpf --manifest-path=Cargo.toml --bpf-out-dir=dist/program


create-keypair:
	solana-keygen new -o keypair.json

setup-install:
	sh -c "$(curl -sSfL https://release.solana.com/v1.9.9/install)"

get-airdrop:
	solana airdrop 1 2rPAnarXLRnRDCkjhpkFUCQ8yqfYu2xcZxYqdgjUok5u --url https://api.devnet.solana.com 

deploy:
	solana deploy --keypair keypair.json dist/program/program.so --url https://api.devnet.solana.com 
