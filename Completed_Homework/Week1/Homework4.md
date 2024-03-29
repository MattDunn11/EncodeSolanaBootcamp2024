# Homework 4 

March 28th 2024

## Sending lamports

Using the command line tool, send your colleagues some lamports. Check your balance and your colleagues balance before and after.

I did not have a colleague's public key readily available so I decided to transfer SOL to my secondary testnet keypair.

My input: 

```solana transfer --from testnetkeypair.json 8Mj8uPjiz7NatBYVSB9sGYGzsJ4w1Hp8xfGJroiEBgM8 0.5 --allow-unfunded-recipient --url https://api.devnet.solana.com --fee-payer testnetkeypair.json```


Output:

```Signature: XYqqpeHRPhjnuLMoGmTywPi81v9vRfsoyZxdQs9zBcCNgJchba9Usgiss56tH3DjbqDq73xRQJxy3GJ3jro3XoU```


Sure enough my secondary testnet wallet recieved 0.5 SOL

![Transfer](../../Images/transfer.png)

## Break Solana Game

In your teams try the [Break Solana](https://break.solana.com/wallet?cluster=devnet) game on one of the Test networks. Post a screen shot of your team's highest score on Discord.

![Break Solana](../../Images/breaksolana.png)

Pretty Cool!

## Fizz Buzz Program

1. Create a project called bootcamp using Cargo
2. The main function should print a welcome message.
3. Write a 'fizz buzz' function that will be called from your main function.
    1. The function should have a loop counting up to 301
    2. If the count is divisible by 3, print "fizz"
    3. If the count is divisible by 5 print "buzz"
4. If the count is divisible by 3 and 5 print "fizz buzz"
5. At the end print the number of times "fizz buzz" occurred.

Using a For Loop inside main to iterate over the range of numbers from 1 through 301 with the variable 'i' we then use the Match Expression to determine if each number is divisible by 3 and 5. If the remainder is 0 for each check it will print Fizz for being divisible by 3 and buzz when divisible by 5. The remainder of 0 is what determins whether to print Fizz, Buzz or Fizz Buzz.

#### Code:

```rust
fn main() {
    println!("Welcome to FizzBuzz!");

    for i in 1..=301 {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", i),
        }
    }
}
```

Output:
```
% cargo run 
   Compiling fizzbuzz v0.1.0 (/Users/Dev/Projects/fizzbuzz)
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/fizzbuzz`
Welcome to FizzBuzz!
1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz
16
17
Fizz
19
Buzz
Fizz
22
23
Fizz
Buzz
26
Fizz
28
29
FizzBuzz
31
32
Fizz
34
Buzz
Fizz
37
38
Fizz
Buzz
41
Fizz
43
44
FizzBuzz
46
47
Fizz
49
Buzz
Fizz
52
53
Fizz
Buzz
56
Fizz
58
59
FizzBuzz
61
62
Fizz
64
Buzz
Fizz
67
68
Fizz
Buzz
71
Fizz
73
74
FizzBuzz
76
77
Fizz
79
Buzz
Fizz
82
83
Fizz
Buzz
86
Fizz
88
89
FizzBuzz
91
92
Fizz
94
Buzz
Fizz
97
98
Fizz
Buzz
101
Fizz
103
104
FizzBuzz
106
107
Fizz
109
Buzz
Fizz
112
113
Fizz
Buzz
116
Fizz
118
119
FizzBuzz
121
122
Fizz
124
Buzz
Fizz
127
128
Fizz
Buzz
131
Fizz
133
134
FizzBuzz
136
137
Fizz
139
Buzz
Fizz
142
143
Fizz
Buzz
146
Fizz
148
149
FizzBuzz
151
152
Fizz
154
Buzz
Fizz
157
158
Fizz
Buzz
161
Fizz
163
164
FizzBuzz
166
167
Fizz
169
Buzz
Fizz
172
173
Fizz
Buzz
176
Fizz
178
179
FizzBuzz
181
182
Fizz
184
Buzz
Fizz
187
188
Fizz
Buzz
191
Fizz
193
194
FizzBuzz
196
197
Fizz
199
Buzz
Fizz
202
203
Fizz
Buzz
206
Fizz
208
209
FizzBuzz
211
212
Fizz
214
Buzz
Fizz
217
218
Fizz
Buzz
221
Fizz
223
224
FizzBuzz
226
227
Fizz
229
Buzz
Fizz
232
233
Fizz
Buzz
236
Fizz
238
239
FizzBuzz
241
242
Fizz
244
Buzz
Fizz
247
248
Fizz
Buzz
251
Fizz
253
254
FizzBuzz
256
257
Fizz
259
Buzz
Fizz
262
263
Fizz
Buzz
266
Fizz
268
269
FizzBuzz
271
272
Fizz
274
Buzz
Fizz
277
278
Fizz
Buzz
281
Fizz
283
284
FizzBuzz
286
287
Fizz
289
Buzz
Fizz
292
293
Fizz
Buzz
296
Fizz
298
299
FizzBuzz
301
```
