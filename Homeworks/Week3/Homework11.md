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

 ```rust
#[account]
pub struct MyAccount {
    pub balance: u64,
}
```

    2. On initialisation, this balance should be set to 100

    ```rust
#[program]
pub mod bitch {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Set the balance of the account to 100
        ctx.accounts.my_account.balance = 100;
        Ok(())
    }
}
```

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)] // Space for the account (8 bytes for discriminator, 8 bytes for balance)
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
    ```

3. Write a test to check that the balance was initialised correctly.

```rust
import { Connection, SystemProgram, PublicKey, Keypair } from '@solana/web3.js';
import { Program } from '@coral-xyz/anchor';
import * as anchor from '@coral-xyz/anchor';
import { Bitch } from '../target/types/bitch';

describe('homework11', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Bitch as Program<homework11>;

  it('Is initialized!', async () => {
    // Generate a keypair for the account
    const myAccount = Keypair.generate();

    // Initialize the connection
    const connection = new Connection(anchor.AnchorProvider.env().connection.rpcEndpoint);

    // Calculate the minimum balance for rent exemption
    const accountSize = 8; // Size of your account data structure in bytes (adjust as needed)
    const lamportsRequired = await connection.getMinimumBalanceForRentExemption(accountSize);

    // Create the account
    const txCreate = await SystemProgram.createAccount({
      fromPubkey: anchor.AnchorProvider.env().wallet.publicKey,
      newAccountPubkey: myAccount.publicKey,
      lamports: lamportsRequired,
      space: accountSize,
      programId: program.programId,
    });

    console.log('Created account with transaction signature:', txCreate);

    // Initialize the account
    const txInitialize = await program.methods.initialize()
      .accounts({
        myAccount: myAccount.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([myAccount])
      .rpc();

    console.log('Your transaction signature for initialization:', txInitialize);

    // Fetch the account data
    const account = await program.account.myAccount.fetch(myAccount.publicKey);

    // Compare the account balance with the expected balance
    const expectedBalance = new anchor.BN(100);
    if (!account.balance.eq(expectedBalance)) {
      console.error('Balance is not equal to 100');
    } else {
      console.log('Balance is correctly initialized to 100');
    }
  });
});

```

At this point I have I have spent close to 20 hours over several days attempting to get this to work. I have studied so hard about utilzing anchor and even chatgpt is incapable of getting this to work. I do not know what the deal is but I have put forth extreme amounts of effort and at this point I am moving on. There is educational value in running my head into the wall for what seemed an eternity but its run out of value. I am running into many errors. I successfully built the project and deployed it, and tested it but its just not coming out perfect and I'm over it at this point. moving on!


Lottery Program

From the Bootcamp repo, [anchor examples](https://github.com/ExtropyIO/SolanaBootcamp/tree/main/examples_anchor)

1. Modify the lottery program so that the payout is only 90% of the total deposits.

2. Add a function that allows lottery admin to withdraw funds after the winner is picked.


Nothing works! This homework is far too ambiguopus for there to be som many errors present. I cannot waste any more of my time.

```
 cargo update
    Updating crates.io index
error: failed to select a version for `solana-program`.
    ... required by package `anchor-lang v0.29.0`
    ... which satisfies dependency `anchor-lang = "^0.29.0"` of package `example1 v0.1.0 (/Users/matt/Repos/SolanaBootcamp/examples_anchor/programs/example1-lottery)`
versions that meet the requirements `^1.16` are: 1.18.11, 1.18.10, 1.18.9, 1.18.8, 1.18.7, 1.18.6, 1.18.5, 1.18.4, 1.18.3, 1.18.2, 1.18.1, 1.18.0, 1.17.31, 1.17.30, 1.17.29, 1.17.28, 1.17.27, 1.17.26, 1.17.25, 1.17.24, 1.17.23, 1.17.22, 1.17.20, 1.17.18, 1.17.17, 1.17.16, 1.17.15, 1.17.14, 1.17.13, 1.17.12, 1.17.11, 1.17.10, 1.17.9, 1.17.8, 1.17.7, 1.17.6, 1.17.5, 1.17.4, 1.17.3, 1.17.2, 1.17.1, 1.17.0, 1.16.27, 1.16.25, 1.16.24, 1.16.23, 1.16.22, 1.16.21, 1.16.20, 1.16.19, 1.16.18, 1.16.17, 1.16.16, 1.16.15, 1.16.14, 1.16.13, 1.16.12, 1.16.11, 1.16.10, 1.16.9, 1.16.8, 1.16.7, 1.16.6, 1.16.5, 1.16.4, 1.16.3, 1.16.2, 1.16.1, 1.16.0

all possible versions conflict with previously selected packages.

  previously selected package `solana-program v1.9.13`
    ... which satisfies dependency `solana-program = "~1.9.13"` of package `anchor-lang v0.24.2`
    ... which satisfies dependency `anchor-lang = "^0.24.2"` of package `example2 v0.1.0 (/Users/matt/Repos/SolanaBootcamp/examples_anchor/programs/example2-rps)`


```