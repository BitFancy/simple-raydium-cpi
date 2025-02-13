use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;

declare_id!("3igNEs6GbvsAgEicsLTkduyhVFWXNkGerQobX1fdZdf9");

#[program]
pub mod amm_cpi {
    use super::*;

    /// swap_base_in instruction
    pub fn proxy_swap_base_in(
        ctx: Context<ProxySwapBaseIn>,
        amount_in: u64,
        minimum_amount_out: u64,
    ) -> Result<()> {
        instructions::swap_base_in(ctx, amount_in, minimum_amount_out)
    }

}
