# solana-course-rareskills

## Day 1

- installation and setup local development environment
- hello world in rust and anchor
- learn about logs and run local validator

## Day 2 
- passing arguements to functions
- arithematic operations in rust 
- underflow and overflow in rust
- default math function in rust unlike solidity

## Day 3

- we can name our default initialize function to anything. 
- anchor will  generates a json file with IDL(Interface defination language)
- you typically use snake_case for function names in rust but compiler will convert it to camelCase in the IDL(JSON) file.
- we can use the IDL file to interact with our program from our frontend. as seen in the test. 
## setup 

Genrate a new keypair for the local validator

```bash
solana-keygen new
```
After all the installation steps 

```bash
solana config set --url localhost
```

In new terminal do to start the local validator

```bash
solana-test-validator
# install rust	
```
open a new terminal and so this to check for longs when we run tests 
```bash
solana logs
```	

## run tests 

```bash
anchor test --skip-local-validator
```
I have a alias setup so it will be
```bash	
atslv

```


