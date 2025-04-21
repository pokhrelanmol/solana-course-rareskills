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
- if struct have a #[derive(Accounts)] then it is an account. and any value with Signer<'info> is a signer and will get added to the accounts array in the IDL file. Still not sure about this.

## Day 4
- similar to error in solidity we have error in rust. 
- when error happend the function will return instead of reverting. 
- we also have require in rust like in solidity. 

## Day 5
- solana program are upgradeable by default. 
- when we deploy a program the program id is same 
- signer can later make this program immutable. 

## Day 6 
- learn some rust vs solidity syntax differences
- learn about structs and how to define them. 
- learn about arrays and how to define them. 
- for dynamic arrays we need to use Vec<T> 
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


