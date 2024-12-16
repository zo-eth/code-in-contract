use anchor_lang::prelude::*;

declare_id!("yourprogramid");

#[program]
pub mod code_in {
    use super::*;

    pub fn user_initialize(ctx: Context<UserInitialize>) -> Result<()> {
        let (expected_pda, expected_bump) = Pubkey::find_program_address(
            &[b"seedhere", ctx.accounts.user.key.as_ref()],
            ctx.program_id,
        );

        let (_expected_db_pda, expected_db_bump) = Pubkey::find_program_address(
            &[b"dbseedhere", ctx.accounts.user.key.as_ref()],
            ctx.program_id,
        );

        require_keys_eq!(
            ctx.accounts.code_account.key(),
            expected_pda,
            ErrorCode::InvalidAccount
        );

        let code_account = &mut ctx.accounts.code_account;
        code_account.bump = expected_bump;
        code_account.code = String::new();
        code_account.method = 0;
        code_account.decode_break = 0;

        let db_account = &mut ctx.accounts.db_account;
        db_account.bump = expected_db_bump;
        db_account.handle = String::new();
        db_account.tail_tx = String::new();
        db_account.type_field = String::new();
        db_account.offset = String::new();

        Ok(())
    }

    pub fn send_code(
        ctx: Context<SendCode>,
        code: String,
        before_tx: String,
        method: u8,
        decode_break: u8,
    ) -> Result<()> {
        let (expected_pda, _expected_bump) = Pubkey::find_program_address(
            &[b"seedhere", ctx.accounts.user.key.as_ref()],
            ctx.program_id,
        );
        if ctx.accounts.code_account.key() != expected_pda {
            return Err(ErrorCode::InvalidAccount.into());
        }
        ctx.accounts.code_account.before_tx = before_tx;
        ctx.accounts.code_account.code = code;
        ctx.accounts.code_account.method = method;
        ctx.accounts.code_account.decode_break = decode_break;
        Ok(())
    }
    pub fn db_code_in(
        ctx: Context<DbCodeIn>,
        handle: String,
        tail_tx: String,
        type_field: String,
        offset: String,
    ) -> Result<()> {
        let _required_lamports = 3_000_000;

        let (expected_pda, _expected_bump) = Pubkey::find_program_address(
            &[b"dbseedhere", ctx.accounts.user.key.as_ref()],
            ctx.program_id,
        );
        if ctx.accounts.db_account.key() != expected_pda {
            return Err(ErrorCode::InvalidAccount.into());
        }

        let expected_receiver = "your wallet"
            .parse::<Pubkey>()
            .map_err(|_| ErrorCode::InvalidWallet)?;

        let _receiver_account = ctx
            .remaining_accounts
            .iter()
            .find(|account| account.key == &expected_receiver);

        let db_pda_account_info = &ctx.accounts.db_account.to_account_info();

        let pda_balance = db_pda_account_info.lamports();
        if pda_balance < _required_lamports {
            return Err(ErrorCode::InvalidTransfer.into());
        }

        if **db_pda_account_info.try_borrow_lamports()? < _required_lamports {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        **db_pda_account_info.try_borrow_mut_lamports()? -= _required_lamports;
        **_receiver_account
            .expect("receiver_account not found")
            .try_borrow_mut_lamports()? += _required_lamports;

        ctx.accounts.db_account.handle = handle;

        ctx.accounts.db_account.tail_tx = tail_tx;
        ctx.accounts.db_account.type_field = type_field;
        ctx.accounts.db_account.offset = offset;
        Ok(())
    }

}
#[account]
pub struct DBaccount {
    pub bump: u8,
    pub handle: String,
    pub tail_tx: String,
    pub type_field: String,
    pub offset: String,
}

#[account]
pub struct CodeAccount {
    pub bump: u8,
    pub decode_break: u8,
    pub method: u8,
    pub code: String,
    pub before_tx: String,
}

#[derive(Accounts)]
pub struct UserInitialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(init, payer = user,seeds = [b"seedhere", user.key().as_ref()],bump,space = 1+1+1+900+100)]
    pub code_account: Account<'info, CodeAccount>,

    #[account(init, payer = user,seeds = [b"dbseedhere", user.key().as_ref()],bump,space =1+20+100+10+50)]
    pub db_account: Account<'info, DBaccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SendCode<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub code_account: Account<'info, CodeAccount>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct DbCodeIn<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub db_account: Account<'info, DBaccount>,
    pub system_program: Program<'info, System>,
}

// 에러 코드 정의
#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds to send code.")]
    InsufficientFunds,
    #[msg("Invalid wallet address.")]
    InvalidWallet,
    #[msg("Invalid receiver address.")]
    InvalidReceiver,
    #[msg("Funds were not received by the expected wallet.")]
    FundsNotReceived,
    #[msg("Provided code account is invalid.")]
    InvalidAccount,
    #[msg("InvalidCodeFormat")]
    InvalidCodeFormat,
    #[msg("InvalidInstructionData")]
    InvalidInstructionData,
    #[msg("InvalidTransfer")]
    InvalidTransfer,
}
