use anchor_lang::prelude::*;

declare_id!("69NSTtF4gFj1Lni6BgVviuTZ7wKmkh1duwCJvqsmmjLn");

#[program]
pub mod day2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,message:String) -> Result<()> {
        msg!("You said: {}", message);
        Ok(())
    }
    
    pub fn array(ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
      msg!("Your array {:?}", arr);
    Ok(()) 
    }
     //@note: this is not working
    pub fn overflow(ctx: Context<Initialize>,a:u64,b:u64) -> Result<()> {
        // let x: u64 = a - b; // will silently overflow if overflow protection is false
        let x: u64 = a.checked_sub(b).unwrap(); // will panic if overflow protection is false
        msg!("You sent {} and {}", a, b);
        msg!("Your result is {}", x);
        Ok(())
    }

  pub fn power(ctx: Context<Initialize>,a:u64,b:u64) -> Result<()> {
        // in solidity we use a ** b 
        let x: u64 = a.pow(b as u32);
        msg!("You sent {} and {}", a, b);
        msg!("powe is {}", x);
        Ok(())
    }
pub fn cube(ctx: Context<Initialize>,a:f64) -> Result<()> {
    // cbrt is not available for u64 so we use f64
        // in solidity we need to use some external lib or do this manually
        let x: f64 = a.cbrt(); // get the cube root
        msg!("You sent {}", a);
        msg!("cube root is {}", x);
        Ok(())
    }
    

}

#[derive(Accounts)]
pub struct Initialize {}
