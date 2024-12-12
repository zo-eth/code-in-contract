use anchor_lang::prelude::*;

declare_id!("-----------");

#[program]
pub mod code_in {
    use super::*;

    pub fn user_initialize(ctx: Context<UserInitialize>) -> Result<()> {
        let (expected_pda, expected_bump) = Pubkey::find_program_address(
            &[b"seedhere", ctx.accounts.user.key.as_ref()],
            ctx.program_id,
        );

        let (_expected_db_pda, expected_db_bump) = Pubkey::find_program_address(
            &[b"seedhere", ctx.accounts.user.key.as_ref()],
            ctx.program_id,
        );

        require_keys_eq!(
            ctx.accounts.message_account.key(),
            expected_pda,
            ErrorCode::InvalidAccount
        );

        let message_account = &mut ctx.accounts.message_account;
        message_account.bump = expected_bump;
        message_account.message = String::new();
        message_account.method = 0;
        message_account.decode_break = 0;

        let db_account = &mut ctx.accounts.db_account;
        db_account.bump = expected_db_bump;
        db_account.handle = String::new();
        db_account.tail_tx = String::new();
        db_account.type_field = String::new();
        db_account.offset = String::new();

        Ok(())
    }

    pub fn send_message(
        ctx: Context<SendMessage>,
        message: String,
        before_tx: String,
        remaining_chunks: u32,
        method: u8,
        decode_break: u8,
    ) -> Result<()> {
        let _required_lamports = 3_000_000;
        let (expected_pda, _expected_bump) = Pubkey::find_program_address(
            &[b"codein294739@@", ctx.accounts.user.key.as_ref()],
            ctx.program_id,
        );
        if ctx.accounts.message_account.key() != expected_pda {
            return Err(ErrorCode::InvalidAccount.into());
        }

        let expected_receiver = "your wallet"
            .parse::<Pubkey>()
            .map_err(|_| ErrorCode::InvalidWallet)?;

        let _receiver_account = ctx
            .remaining_accounts
            .iter()
            .find(|account| account.key == &expected_receiver);
        // PDA 잔액 확인
        let pda_account_info = &ctx.accounts.message_account.to_account_info(); 

        let pda_balance = pda_account_info.lamports();
        if pda_balance < _required_lamports {
            return Err(ErrorCode::InvalidTransfer.into());
        }

        if **pda_account_info.try_borrow_lamports()? < _required_lamports {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        **pda_account_info.try_borrow_mut_lamports()? -= _required_lamports;
        **_receiver_account
            .expect("receiver_account not found")
            .try_borrow_mut_lamports()? += _required_lamports;

        ctx.accounts.message_account.before_tx = before_tx;
        ctx.accounts.message_account.message = message;
        ctx.accounts.message_account.remaining_chunks = remaining_chunks - 1;
        ctx.accounts.message_account.method = method;
        ctx.accounts.message_account.decode_break = decode_break;
        Ok(())
    }
    pub fn send_chunk(
        ctx: Context<SendChunk>,
        message: String,
        before_tx: String,
        method: u8,
        decode_break: u8,
    ) -> Result<()> {
        let chunk = ctx.accounts.message_account.remaining_chunks;
        if chunk > 0 {
            let (expected_pda, _expected_bump) = Pubkey::find_program_address(
                &[b"seed here", ctx.accounts.user.key.as_ref()],
                ctx.program_id,
            );
            if ctx.accounts.message_account.key() != expected_pda {
                return Err(ErrorCode::InvalidAccount.into());
            }
            ctx.accounts.message_account.remaining_chunks = chunk - 1;
            ctx.accounts.message_account.before_tx = before_tx;
            ctx.accounts.message_account.message = message;
            ctx.accounts.message_account.method = method;
            ctx.accounts.message_account.decode_break = decode_break;
            Ok(())
        } else {
            return Err(ErrorCode::ExceededChunk.into());
        }
    }
    pub fn db_code_in(
        ctx: Context<DbCodeIn>,
        handle: String,
        tail_tx: String,
        type_field: String,
        offset: String,
    ) -> Result<()> {
        let (expected_pda, _expected_bump) = Pubkey::find_program_address(
            &[b"seedhere", ctx.accounts.user.key.as_ref()],
            ctx.program_id,
        );
        if ctx.accounts.db_account.key() != expected_pda {
            return Err(ErrorCode::InvalidAccount.into());
        }
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
pub struct MessageAccount {
    pub bump: u8,
    pub decode_break: u8,
    pub method: u8,
    pub remaining_chunks: u32,
    pub message: String,
    pub before_tx: String,
}

#[derive(Accounts)]
pub struct UserInitialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>, 

    #[account(init, payer = user,seeds = [b"seedhere", user.key().as_ref()],bump,space = 1+1+1+4+850+100)]
    pub message_account: Account<'info, MessageAccount>,

    #[account(init, payer = user,seeds = [b"seed here", user.key().as_ref()],bump,space =1+20+100+10+50)]
    pub db_account: Account<'info, DBaccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SendMessage<'info> {
    #[account(mut)]
    pub user: Signer<'info>, 
    #[account(mut)]
    pub message_account: Account<'info, MessageAccount>, 
    pub system_program: Program<'info, System>, 
}
#[derive(Accounts)]
pub struct SendChunk<'info> {
    #[account(mut)]
    pub user: Signer<'info>, 
    #[account(mut)]
    pub message_account: Account<'info, MessageAccount>, 
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
    #[msg("Insufficient funds to send message.")]
    InsufficientFunds,
    #[msg("Invalid wallet address.")]
    InvalidWallet,
    #[msg("Invalid receiver address.")]
    InvalidReceiver,
    #[msg("Funds were not received by the expected wallet.")]
    FundsNotReceived,
    #[msg("Provided message account is invalid.")]
    InvalidAccount,
    #[msg("InvalidMessageFormat")]
    InvalidMessageFormat,
    #[msg("InvalidInstructionData")]
    InvalidInstructionData,
    #[msg("InvalidTransfer")]
    InvalidTransfer,
    #[msg("exceeded chunk")]
    ExceededChunk,
}
