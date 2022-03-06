Deployment
Solana has three main clusters: mainnet-beta, devnet, and testnet. For developers, devnet and mainnet-beta are the most interesting. devnet is where you test your application in a more realistic environment than localnet. testnet is mostly for validators.

We are going to deploy on devnet.

Here is your deployment checklist 🚀

Run anchor build. Your program keypair is now in target/deploy. Keep this secret. You can reuse it on all clusters.
Run solana address -k target/deploy/tic_tac_toe-keypair.json and copy the address into your declare_id! macro at the top of lib.rs.
Run anchor build again. This step is necessary to include our new program id in the binary.
Change the provider.cluster variable in Anchor.toml to devnet.
Run anchor deploy
Run anchor test
There is more to deployments than this e.g. understanding how the BPFLoader works, how to manage keys, how to upgrade your programs and more. Keep reading to learn more!

Well done! You've finished the essentials section. You can now move on to the more advanced parts of Anchor.