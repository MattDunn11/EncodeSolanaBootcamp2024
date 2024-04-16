# Homework 11

1. Use the Anchor command line tools to create a new project.
```
% anchor init homework11
yarn install v1.22.22
warning package.json: No license field
info No lockfile found.
warning No license field
[1/4] 🔍  Resolving packages...
[2/4] 🚚  Fetching packages...
[3/4] 🔗  Linking dependencies...
[4/4] 🔨  Building fresh packages...
success Saved lockfile.
✨  Done in 5.19s.
Initialized empty Git repository in /Users/matt/Repos/SolanaBootcamp/homework11/.git/
homework11 initialized
```

2. Adapt the default program as follows

    1. In an account we want to store a balance of type u64

    2. On initialisation, this balance should be set to 100

    src/lib.rs :

```rust
    use anchor_lang::prelude::*;

declare_id!("ExTzqt8Uk8Drkx8Tbj3tPTAJwYyPtGgVKkQzcNYY3pGn");

#[program]
pub mod storage {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, balance: u64) -> Result<()> {
        ctx.accounts.storage_account.balance = balance;
        msg!("Changed data to: {}!", balance);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds=[signer.key().as_ref()],
        bump,
        payer = signer,
        space = 8 + 8
    )]
    pub storage_account: Account<'info, StorageData>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StorageData {
    balance: u64,
}
    ```


3. Write a test to check that the balance was initialised correctly.

tests/tests.ts :

```rust
import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Storage } from '../target/types/storage';
import { expect } from 'chai';
import { before } from 'mocha';

// Configure the client to use the local cluster.
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.Storage as Program<Storage>;
const systemProgram = anchor.web3.SystemProgram.programId;
let storageDataPDA;

describe('storage', async () => {
	it('Intialize', async () => {
		try {
			const data = new anchor.BN(100);
			[storageDataPDA] = await anchor.web3.PublicKey.findProgramAddressSync(
				[provider.publicKey.toBuffer()],
				program.programId
			);
			const tx = await program.methods
				.initialize(data)
				.accounts({
					storageAccount: storageDataPDA,
					systemProgram: systemProgram,
				})
				.signers([])
				.rpc();
			console.log('🚀 Intialization transaction:', tx);
		} catch (error) {
			console.error(
				'Check if pda is already in use after initializing account',
				error.logs[3]
			);
		}
	});
	it('Account data is initialized to 100', async () => {
		// fetch data for pda
		const data = await program.account.storageData.fetch(storageDataPDA);
		// convert BN to decimal
		const value = parseInt(data.balance.toString('hex'), 16);
		expect(value).equal(100, 'Value is not 100');
	});
});
```

