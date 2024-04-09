# Homework 10

#### 1. Try a simple client transaction in Solana playground [https://beta.solpg.io/](https://beta.solpg.io/)

1. Make sure you are connected to the devnet and you have a wallet set up
![Connected](../../Images/Connected.png)

2. Run the default client code, this will tell you your balance.
```
Building...
Running client...
  client.ts:
    My address: FUufJrZsAySQT5TBi3CK2DsZ7Ym2cmoUYPEUGMdQfHj9
    My balance: 7.35784809 SOL


Build successful. Completed in 1.37s.
```
3. Create an airdrop signature and request the airdrop from the connection object 
pg.connection.requestAirdrop you will need to add your public key and the number of lamports you want.

4. Use await pg.connection.confirmTransaction; to
confirm the transaction.

#### 2. Investigating Dapp Scaffold

Try following the functionality it provides:

1. Make sure your wallet is connected to the dev network

2. Try the airdrop to give yourself some SOL

3. Try to sign a message

Try altering the code to send a transaction to
send to a hardcoded address

