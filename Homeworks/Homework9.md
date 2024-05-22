# Homework 9

Make sure you are using an older version of the solana-cli before you try to build the examples
```
sh -c "$(curl -sSfL https://release.solana.com/v1.16.21/install)
```
Done!
```
% solana --version
solana-cli 1.16.21 (src:7fa07719; feat:4033350765, client:SolanaLabs)
```
1. Work through the CPI example : example4-
cpi. You can run it with
npm run build
npm run deploy:4
npm run call:4
This program receives no instruction data, no accounts and it only logs a message usingthe msg! macro that can be viewed in the window running `solana logs

```
logs: [
    'Program FXgYQYULHeyPvz8s3UPnwXUhg1LdQqnPUxmxgZipB3v invoke [1]',
    'Program log: [entrypoint] CPI',
    'Program log: [entrypoint] Calling helloworld',
    'Program FXgYQYULHeyPvz8s3UPnwXUhg1LdQqnPUxmxgZipB3v consumed 1525 of 200000 compute units',
    'Program FXgYQYULHeyPvz8s3UPnwXUhg1LdQqnPUxmxgZipB3v failed: Invoked an instruction with too many accounts (11535677830091389790 > 255)'
```

Notes: Moving on per instructors notes as something is nor right with the example.

2. Work through the compute budget example :
example5-compute. You can run it with
( npm run build )
npm run deploy:5
npm run call:5

```
npm run call:5

FrZBwX7Fvei5JL5ZHyEpdTq7EJn7YPegKy2CxoecncGH owns:


It cost:
	0 SOL
	0 Lamports
to perform the call
```
3. Work through the PDA example : example6-
pda
You can run it with
( npm run build )
npm run deploy:6
npm run call:6

```
 logs: [
    'Program FrZBwX7Fvei5JL5ZHyEpdTq7EJn7YPegKy2CxoecncGH invoke [1]',
    'Program log: [entrypoint] multifunc example entrypoint',
    'Program log: [instruction] Total payload: [0, 1, 113, 255, 1]',
    'Program log: [instruction] Received function flag: 0',
    'Program log: [instruction] Initialising PDA',
    'Program log: [instruction] extracted seed: "q"',
    'Program log: [instruction] extracted bump: 255',
    'Program log: [instruction] extracted account size: 1',
    'Program log: [processor] Received instruction struct: PdaCreate { seed: "q", bump: 255, account_size: 1 }',
    'Program log: [functions] 51B2HcuEVfswfJzE8Casfx7uFGTmkSa43Qkd5x5b8zsk will pay to initalise PDA at 2bD95mQKLAh6RNiUua45LqZGFvjpy7ox1egUsAPbWHA4',
    'Program log: The account has 0 lamports',
    'Program log: [functions] PDA instruction created',
    'Program FrZBwX7Fvei5JL5ZHyEpdTq7EJn7YPegKy2CxoecncGH consumed 33641 of 200000 compute units',
    'Program FrZBwX7Fvei5JL5ZHyEpdTq7EJn7YPegKy2CxoecncGH failed: Invoked an instruction with data that is too large (12884933187 > 10240)'
  ]
```
Notes: Moving on per instructors notes as something is nor right with the example.

4. Anchor : Create a hello world project, follow the instructions [here](https://book.anchor-lang.com/getting_started/hello_anchor.html)

```
% anchor init Hello_Anchor
yarn install v1.22.22
warning package.json: No license field
info No lockfile found.
warning No license field
[1/4] 🔍  Resolving packages...
[2/4] 🚚  Fetching packages...
[3/4] 🔗  Linking dependencies...
[4/4] 🔨  Building fresh packages...
success Saved lockfile.
✨  Done in 9.80s.
Initialized empty Git repository in /Users/matt/Repos/SolanaBootcamp/hello-anchor/.git/
hello-anchor initialized
```