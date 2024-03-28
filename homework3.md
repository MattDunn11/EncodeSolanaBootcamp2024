Homework 3

**Installing Solana Command Line Tools**

    sh -c "$(curl -sSfL https://release.solana.com/v1.18.4/install)"

Installed!

    keypair % solana --version solana-cli 1.18.4 (src:356c6a38; feat:3352961542, client:SolanaLabs)

**Create a Keypair**

Instructions: 
    mkdir ~/my-solana-wallet-solana-keygen new --outfile ~/my-solanawallet/my-keypair.json

Actual input: 
    mkdir MainTestNetWallet

Actual Input: 
    cd MaintestNetwallet

Actual Input: 
    MaintestNetwallet % solana-keygen new --outfile testnetkeypair.json


Output: 
    Generating a new keypair

    For added security, enter a BIP39 passphrase

    NOTE! This passphrase improves security of the recovery seed phrase NOT the
    keypair file itself, which is stored as insecure plain text

    BIP39 Passphrase (empty for none): 

    Wrote new keypair to testnetkeypair.json
    =========================================================================
    pubkey: 69LmXrZWBKUvsdYAjowZ5yhtJm2QzLrJeXvsbDeN5Qpt
    =========================================================================
    Omitting private seed phrase

**Display the result with**
    solana-keygen pubkey ~/my-solana-wallet/mykeypair.json

Actual Input: 
    solana-keygen pubkey testnetkeypair.json

Output: 
    69LmXrZWBKUvsdYAjowZ5yhtJm2QzLrJeXvsbDeN5Qpt


**Verify your address**
    solana-keygen verify <PUBKEY> ~/my-solanawallet/my-keypair.json

Actual input: 
    solana-keygen verify 69LmXrZWBKUvsdYAjowZ5yhtJm2QzLrJeXvsbDeN5Qpt testnetkeypair.json

Output: 
    Verification for public key: 69LmXrZWBKUvsdYAjowZ5yhtJm2QzLrJeXvsbDeN5Qpt: Success

**Connect to the dev network**
    solana config set --url https://api.devnet.solana.com

Actual input: 
    solana config set --url https://api.devnet.solana.com

Output: 
    Config File: /Users/matt/.config/solana/cli/config.yml
    RPC URL: https://api.devnet.solana.com 
    WebSocket URL: wss://api.devnet.solana.com/ (computed)
    Keypair Path: /Users/matt/.config/solana/id.json 
    Commitment: confirmed 

**You can check the configuration with**
    Solana config get

Actual input: 
    Solana config get

Output: 
    Config File: /Users/matt/.config/solana/cli/config.yml
    RPC URL: https://api.devnet.solana.com 
    WebSocket URL: wss://api.devnet.solana.com/ (computed)
    Keypair Path: /Users/matt/.config/solana/id.json 
    Commitment: confirmed

**Get some tokens from dev net**
    solana airdrop 1 <RECIPIENT_ACCOUNT_ADDRESS> --url https://api.devnet.solana.com

Actual input: 
    solana airdrop 1 69LmXrZWBKUvsdYAjowZ5yhtJm2QzLrJeXvsbDeN5Qpt --url https://api.devnet.solana.com

Output: 
    Requesting airdrop of 1 SOL
    Error: airdrop request failed. This can happen when the rate limit is reached.

**If at first you don't succeed, try and try again!**

Actual input: 
    solana airdrop 1 69LmXrZWBKUvsdYAjowZ5yhtJm2QzLrJeXvsbDeN5Qpt --url https://api.devnet.solana.com

Output:
    Requesting airdrop of 1 SOL

    Signature: qCtUMbkhyfi2Zx7e4afN3ENmTLLjJrYYPd2M6UiN9zB6ctkSTvSaMx7r8ZZ3gDa8zwCHqx6ubGTmHpoyMzsgZy7

    1 SOL

Hooray!

**You can also check your balance with** 
    
    'solana balance <ACCOUNT_ADDRESS> --url https://api.devnet.solana.com'

Actual input: 
    
    'solana balance 69LmXrZWBKUvsdYAjowZ5yhtJm2QzLrJeXvsbDeN5Qpt --url https://api.devnet.solana.com'
   


Output: 
    1 SOL