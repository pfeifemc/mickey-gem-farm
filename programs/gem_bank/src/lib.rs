use anchor_lang::prelude::*;
use instructions::*;

declare_id!("3zgGJ7xjcEjr294KcDnxLgbDQTQoArd9ReY7DzXqPbUk");

pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

#[program]
pub mod gem_bank {
    use super::*;

    pub fn init_bank(ctx: Context<InitBank>) -> ProgramResult {
        instructions::init_bank::handler(ctx)
    }

    pub fn set_bank_flags(ctx: Context<SetBankFlags>, flags: u8) -> ProgramResult {
        instructions::set_bank_flags::handler(ctx, flags)
    }

    pub fn init_vault(
        ctx: Context<InitVault>,
        _bump: u8,
        owner: Pubkey,
        name: String,
    ) -> ProgramResult {
        instructions::init_vault::handler(ctx, owner, name)
    }

    pub fn set_vault_lock(ctx: Context<UnlockVault>, vault_lock: bool) -> ProgramResult {
        instructions::set_vault_lock::handler(ctx, vault_lock)
    }

    pub fn update_vault_owner(ctx: Context<UpdateVaultOwner>, new_owner: Pubkey) -> ProgramResult {
        instructions::update_vault_owner::handler(ctx, new_owner)
    }

    pub fn deposit_gem(
        ctx: Context<DepositGem>,
        _bump_gem_box: u8,
        _bump_gdr: u8,
        amount: u64,
    ) -> ProgramResult {
        instructions::deposit_gem::handler(ctx, amount)
    }

    pub fn withdraw_gem(ctx: Context<WithdrawGem>, _bump: u8, amount: u64) -> ProgramResult {
        instructions::withdraw_gem::handler(ctx, amount)
    }

    pub fn add_to_whitelist(
        ctx: Context<AddToWhitelist>,
        _bump: u8,
        whitelist_type: u8,
    ) -> ProgramResult {
        instructions::add_to_whitelist::handler(ctx, whitelist_type)
    }

    pub fn remove_from_whitelist(ctx: Context<RemoveFromWhitelist>, _bump: u8) -> ProgramResult {
        instructions::remove_from_whitelist::handler(ctx)
    }
}