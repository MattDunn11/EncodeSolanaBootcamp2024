# Homework 8

## Part 1 : Solana Tokens

#### 1. Follow the instructions from the lesson and use the spl-token-cli to create
#### a) A fungible token with a supply of 10,000
#### b) An NFT

#### 2. Try sending these tokens to others in your team , and use the command line to find details about the tokens.

#### 3. Try sending using both the transfer and transfer --fund-recipient approach.

```
% spl-token create-token
Creating token 4SF6t8tj2ifePTCbWWJWnvT4gM9oBfFfm5FRiwodFV7p under program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA

Address:  4SF6t8tj2ifePTCbWWJWnvT4gM9oBfFfm5FRiwodFV7p
Decimals:  9

Signature: 5Wttryc1T3SAQxb8susKNmBcRDaCtMKEiuDUAfoBDhobYjbaD3zB6UCgky12w5Ho8DLJYM9UXh7LdvNwBo5sGKP8

% spl-token create-account 4SF6t8tj2ifePTCbWWJWnvT4gM9oBfFfm5FRiwodFV7p
Creating account 1uS7ii6HZyvV1uhqG7WahL2KAKR4KTAMH6V5Vy626g8

Signature: 632nYXGXrLtEkwKsEfFUAkMYJkQ2tgCL6NuMAP55sk8Xq8WEskjeorXETZ6eZv4MYyiPxP2mwPczBQourJqmsTqr

% spl-token mint 4SF6t8tj2ifePTCbWWJWnvT4gM9oBfFfm5FRiwodFV7p 10000
Minting 10000 tokens
  Token: 4SF6t8tj2ifePTCbWWJWnvT4gM9oBfFfm5FRiwodFV7p
  Recipient: 1uS7ii6HZyvV1uhqG7WahL2KAKR4KTAMH6V5Vy626g8

Signature: 2jsRyhjpDDfAFLzgA73MBerojpytNSKc43T4Fq9Q3M1VxeSYtqc8hrw7bLG1PZQ88JCkpUNP638qm9sRURMotqZr

% spl-token supply 4SF6t8tj2ifePTCbWWJWnvT4gM9oBfFfm5FRiwodFV7p
10000

% spl-token accounts 4SF6t8tj2ifePTCbWWJWnvT4gM9oBfFfm5FRiwodFV7p
Balance
-------
10000

% spl-token transfer 4SF6t8tj2ifePTCbWWJWnvT4gM9oBfFfm5FRiwodFV7p 500 24CFvjS6FE3LxZyKNVhGxXXV6N2zp2TWwP4eQ3ktV4XM
Transfer 500 tokens
  Sender: 1uS7ii6HZyvV1uhqG7WahL2KAKR4KTAMH6V5Vy626g8
  Recipient: 24CFvjS6FE3LxZyKNVhGxXXV6N2zp2TWwP4eQ3ktV4XM
  Recipient associated token account: DGNWzcarVGYW8HMKW8iY7LJVoPbfaHcL8oPnzgcHW1jY
Error: "Error: Recipient's associated token account does not exist. Add `--fund-recipient` to fund their account"

% spl-token transfer 4SF6t8tj2ifePTCbWWJWnvT4gM9oBfFfm5FRiwodFV7p 500 24CFvjS6FE3LxZyKNVhGxXXV6N2zp2TWwP4eQ3ktV4XM --fund-recipient
Transfer 500 tokens
  Sender: 1uS7ii6HZyvV1uhqG7WahL2KAKR4KTAMH6V5Vy626g8
  Recipient: 24CFvjS6FE3LxZyKNVhGxXXV6N2zp2TWwP4eQ3ktV4XM
  Recipient associated token account: DGNWzcarVGYW8HMKW8iY7LJVoPbfaHcL8oPnzgcHW1jY
  Funding recipient: DGNWzcarVGYW8HMKW8iY7LJVoPbfaHcL8oPnzgcHW1jY

Signature: 5KY8YuLVb7nh3peCufCS6sHcg2U4f562EJ5kVRfzZd9wpi3DoqCabFcefyjxeTDUf9w4PNNaYkwYXM5qVa5ScGvf
```
#### An NFT
```
% spl-token create-token --decimals 0
Creating token xUbEksCfkEjTN85Jm6aeQPhWUA3Sap9qpEdkdZ3GUHD under program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA

Address:  xUbEksCfkEjTN85Jm6aeQPhWUA3Sap9qpEdkdZ3GUHD
Decimals:  0

Signature: ruboYkmpeiC5hDJ9YEBcWHVTq2Gb5aRXUQ4JyBbwHgyCVMjNGTeBs4h9RgqFLLHpkgfioZFkL23nXNvS7Dyqxk8

% spl-token create-account xUbEksCfkEjTN85Jm6aeQPhWUA3Sap9qpEdkdZ3GUHD
Creating account EZXzCnfdMAG6ofigTm7uK7BDGSpx1JDwQaQyQZzjvA1Z

Signature: b1jgQgS5LhgZQSGNTQrXNePZBBv6fyq3ehEXK51gfszJyN1WnYrqjfyH4C5X6gHPTmy8MqqsfviaYfmaDYzcTww

% spl-token mint xUbEksCfkEjTN85Jm6aeQPhWUA3Sap9qpEdkdZ3GUHD 1 EZXzCnfdMAG6ofigTm7uK7BDGSpx1JDwQaQyQZzjvA1Z
Minting 1 tokens
  Token: xUbEksCfkEjTN85Jm6aeQPhWUA3Sap9qpEdkdZ3GUHD
  Recipient: EZXzCnfdMAG6ofigTm7uK7BDGSpx1JDwQaQyQZzjvA1Z

Signature: 5uYhiXXTRL1xnRX9Jx6bzUANSxEjXWYvRkdA7VA3hum6j1cfj8USQQv2Drbn5r2iBSZ7fNST32dnjFPK5An1pMC8

% spl-token authorize xUbEksCfkEjTN85Jm6aeQPhWUA3Sap9qpEdkdZ3GUHD mint --disable
Updating xUbEksCfkEjTN85Jm6aeQPhWUA3Sap9qpEdkdZ3GUHD
  Current mint: FUufJrZsAySQT5TBi3CK2DsZ7Ym2cmoUYPEUGMdQfHj9
  New mint: disabled

Signature: 5o5R6Hou3HSBWeqFZSpcyrErTtJ7j3tpBQkg4Js57EXy84SRQC3rWvHtMrx1HJGhUgYtgVtvPnFg1yjmDv9pYpVD

% spl-token account-info xUbEksCfkEjTN85Jm6aeQPhWUA3Sap9qpEdkdZ3GUHD

SPL Token Account
  Address: EZXzCnfdMAG6ofigTm7uK7BDGSpx1JDwQaQyQZzjvA1Z
  Program: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
  Balance: 1
  Decimals: 0
  Mint: xUbEksCfkEjTN85Jm6aeQPhWUA3Sap9qpEdkdZ3GUHD
  Owner: FUufJrZsAySQT5TBi3CK2DsZ7Ym2cmoUYPEUGMdQfHj9
  State: Initialized
  Delegation: (not set)
  Close authority: (not set)

  % spl-token supply xUbEksCfkEjTN85Jm6aeQPhWUA3Sap9qpEdkdZ3GUHD
1

% spl-token transfer --fund-recipient xUbEksCfkEjTN85Jm6aeQPhWUA3Sap9qpEdkdZ3GUHD 1 24CFvjS6FE3LxZyKNVhGxXXV6N2zp2TWwP4eQ3ktV4XM
Transfer 1 tokens
  Sender: EZXzCnfdMAG6ofigTm7uK7BDGSpx1JDwQaQyQZzjvA1Z
  Recipient: 24CFvjS6FE3LxZyKNVhGxXXV6N2zp2TWwP4eQ3ktV4XM
  Recipient associated token account: 6aJ2W7GkixZQjX4gRReE5Fy1u3TywdJsTjq6UMCs9DY2
  Funding recipient: 6aJ2W7GkixZQjX4gRReE5Fy1u3TywdJsTjq6UMCs9DY2

Signature: 4ENTxaPev8e2R9LB5k2yvzuWmVEbcs69zNZgAxVcLU9nqPnan3RhsPoRhReQckF16BwmzqGErauMu4CMvJN74dZ4
```


## Part 1 : Solana Programs

#### Using the examples in the [repo](https://github.com/ExtropyIO/SolanaBootcamp/tree/main/examples_baremetal)
#### 1. Modify the existing msg! in example1-helloworld to log the program ID.
#### 2. Retrieve the total program size ofexample1-helloworld.
#### 3. Retrieve the lamport balance of example2-counter.
#### 4. Modify the client for example2-counter to feed an incorrect address for the greeting

### Example 1

#### Original code for lib.rs
```
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    _program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    _accounts: &[AccountInfo], // accounts to not interact with (this time)
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("[lib] Hello World Rust program entrypoint");

    Ok(())
}
```

#### Modify the existing msg! in example1-helloworld to log the program ID.

#### New code with changes:
```
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
     program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    _accounts: &[AccountInfo], // accounts to not interact with (this time)
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("[lib] Program ID: {:?}", program_id);
    msg!("[lib] Hello World Rust program entrypoint");

    Ok(())
}
```
### 