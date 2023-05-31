use anchor_lang::prelude::*;

declare_id!("5zZ2kGW8eur13REqFXQK2tYch54a5K3rpR372mFG7woc");

#[program]
pub mod tiny_adventure {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.new_game_data_account.player_position_x = 0;
        msg!("A Journey Begins!");
        msg!("..\0/..");
        Ok(())
    }

    pub fn move_left(ctx: Context<MoveLeft>) -> Result<()> {
        let game_data_account = &mut ctx.accounts.game_data_account;
        if game_data_account.player_position_x == 0 {
            msg!("You are back at the start.");
        } else {
            game_data_account.player_position_x -= 1;
            print_player(game_data_account.player_position_x);
        }
        Ok(())
    }

    pub fn move_right(ctx: Context<MoveRight>) -> Result<()> {
        let game_data_account = &mut ctx.accounts.game_data_account;
        if game_data_account.player_position_x == 0 {
            msg!("You are back at the start.");
        } else {
            game_data_account.player_position_x -= 1;
            print_player(game_data_account.player_position_x);
        }
        Ok(())
    }

    pub fn move_up(ctx: Context<MoveUp>) -> Result<()> {
        let game_data_account = &mut ctx.accounts.game_data_account;
        if game_data_account.player_position_y == 3 {
            msg!("You have reached the end! Super!");
        } else {
            game_data_account.player_position_y = game_data_account.player_position_y + 1;
            print_player(game_data_account.player_position_y);
        }
        Ok(())
    }

    pub fn move_down(ctx: Context<MoveDown>) -> Result<()> {
        let game_data_account = &mut ctx.accounts.game_data_account;
        if game_data_account.player_position_y == 3 {
            msg!("You have reached the end! Super!");
        } else {
            game_data_account.player_position_y = game_data_account.player_position_y + 1;
            print_player(game_data_account.player_position_y);
        }
        Ok(())
    }
}

fn print_player(player_position: u8) {
    if player_position == 0 {
        msg!("A Journey Begins!");
        msg!("<-.......");
    } else if player_position == 1 {
        msg!("...^....");
    } else if player_position == 2 {
        msg!("....->...");
    } else if player_position == 3 {
        msg!("....v....");
        msg!("You have reached the end! Super!");
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init_if_needed,
        seeds = [b"level1"],
        bump,
        payer = signer,
        space = 8 + 1
    )]
    pub new_game_data_account: Account<'info, GameDataAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MoveLeft<'info> {
    #[account(mut)]
    pub game_data_account: Account<'info, GameDataAccount>,
}

#[derive(Accounts)]
pub struct MoveRight<'info> {
    #[account(mut)]
    pub game_data_account: Account<'info, GameDataAccount>,
}

#[derive(Accounts)]
pub struct MoveUp<'info> {
    #[account(mut)]
    pub game_data_account: Account<'info, GameDataAccount>,
}

#[derive(Accounts)]
pub struct MoveDown<'info> {
    #[account(mut)]
    pub game_data_account: Account<'info, GameDataAccount>,
}

#[account]
pub struct GameDataAccount {
    player_position_x: u8,
    player_position_y: u8,
}
