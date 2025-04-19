# solana-course-rareskills

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


