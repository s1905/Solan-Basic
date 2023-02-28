use anchor_lang::prelude::*;

declare_id!("BWZZMkhTmnFfDc746uf6tAAMFeu2SD3c5e9y8mbvSpic"); // to get the program id run - solana address -k "target/deploy/notes-keypair.json"

#[account] // account attribute
pub struct Note {
    pub content: String,
    pub user:  Pubkey //Owner of note
}

#[derive(Accounts)] // in here we need to pass in all the accounts that will be used by the function defined in #[program], remember everything in solana is an account
pub struct CreateNote<'info> {
    // for context
    #[account(init, payer = user, space = 2000)] // inside this attribute we need to specify what can be done with the note
    pub note: Account<'info, Note>,

    // for user
    #[account(mut)]
    pub user: Signer<'info>,

    // whenever we try to initialize any program we need to ask system program to do so
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct DeleteNote<'info> {
    #[account(mut, has_one = user, close = user)]
    
}

#[program] // entry point of our smart contract, we write all the functions that can be called from frontend here
pub mod notes {
    use super::*;

    pub fn create_note(ctx: Context<CreateNote>, content: String) -> Result<()> {
        let note = &mut ctx.accounts.note;
        let user = &mut ctx.accounts.user;

        note.content = content;
        note.user = *user.key;

        Ok(())
    }
}
