use anchor_lang::prelude::*;

declare_id!("2h8WcsK5ih7BUqBbpLroF1AMQ2ZwBniBPr1N2afGR4Wj");

#[program]
pub mod anchor_function_tutorial {
    use super::*;

    
pub fn boaty_mc_boatface(ctx: Context<Initialize>,yo:String) -> Result<()> {
    Ok(())
}

pub fn add(ctx: Context<Initialize>,a:u64,b:u64) -> Result<()> {
    let sum = a+b;
    msg!("The sum of {} and {} is {}",a,b,sum);
    Ok(())
}

pub fn sub(ctx: Context<Initialize>,a:u64,b:u64) -> Result<()> {
    let diff = a-b;
    msg!("The difference of {} and {} is {}",a,b,diff);
    Ok(())
}

pub fn mul(ctx: Context<Initialize>,a:u64,b:u64) -> Result<()> {
    let product = a*b;
    msg!("The product of {} and {} is {}",a,b,product);
    Ok(())
}

pub fn div(ctx: Context<Initialize>,a:u64,b:u64) -> Result<()> {
    let quotient = a/b;
    msg!("The quotient of {} and {} is {}",a,b,quotient);
    Ok(())
}

pub fn modulo(ctx: Context<Initialize>,a:u64,b:u64) -> Result<()> {
    let remainder = a%b;
    msg!("The remainder of {} and {} is {}",a,b,remainder);
    Ok(())
}



}

#[derive(Accounts)]
pub struct Initialize {
}
