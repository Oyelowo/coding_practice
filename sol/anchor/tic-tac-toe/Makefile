build:
	anchor build

test:
	anchor test

deploy-init:
# Run anchor build. Your program keypair is now in target/deploy. Keep this secret. You can reuse it on all clusters.
	make build
# Run solana address -k target/deploy/tic_tac_toe-keypair.json and copy the address into your declare_id! macro at the top of lib.rs.
	solana address -k target/deploy/tic_tac_toe-keypair.json 

deploy:
# Before this, run anchor build again. This step is necessary to include our new program id in the binary.
# Change the provider.cluster variable in Anchor.toml to devnet.
	anchor deploy
	anchor test