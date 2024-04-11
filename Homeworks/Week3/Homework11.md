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

3. Write a test to check that the balance was initialised correctly.

Lottery Program

From the Bootcamp repo, [anchor examples](https://github.com/ExtropyIO/SolanaBootcamp/tree/main/examples_anchor)

1. Modify the lottery program so that the payout is only 90% of the total deposits.

2. Add a function that allows lottery admin to withdraw funds after the winner is picked.