use anchor_lang::prelude::*;

declare_id!("2h8WcsK5ih7BUqBbpLroF1AMQ2ZwBniBPr1N2afGR4Wj");

#[program]
pub mod anchor_function_tutorial {
    use super::*;

    
pub fn boaty_mc_boatface(ctx: Context<Initialize>) -> Result<()> {
    Ok(())
}

}
#[derive(Accounts)]
pub struct Initialize<'info> {
    signer: Signer<'info>,
    another_signer: Signer<'info>,
}
