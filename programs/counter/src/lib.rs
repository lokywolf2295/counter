use anchor_lang::prelude::*;

// Esta es la clave pública de tu programa y se actualizará automáticamente cuando construyas el proyecto.
declare_id!("4tFoBRn2KreUN5wemJXiHs1pAjvhMt4cmEJwUSNKmNLB");

#[program] //lo convierte en un programa
mod counter {
    use super::*; //utilizar todo lo que está en el modulo padre
    
    pub fn create_counter(ctx: Context<Create>, number: u64) -> Result<()> { //Result devuelve si todo salió bien o si falló
        ctx.accounts.counter.number = number;
        ctx.accounts.counter.authority = ctx.accounts.authority.key();
        msg!("Creando un contador con numero inicial: {}!", number); // El mensaje aparecerá en los registros tx
        Ok(()) //siempre va esta linea al final de un programa de anchor
    }

    pub fn delete_counter(_ctx: Context<Delete>) -> Result<()> {
        msg!("Contador Eliminado");
        Ok(())
    }

    pub fn update_counter(ctx: Context<Update>, new_number: u64) -> Result<()> {
        ctx.accounts.counter.number = new_number;
        msg!("Actualizando el contador al nuevo numero: {}!", new_number);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info>{
    // Debemos especificar el espacio para inicializar una cuenta.
    // Los primeros 8 bytes son el discriminador de cuenta por defecto,
    // los siguientes 8 bytes vienen de Counter.numero siendo de tipo u64.
    // los 32 restantes son de la publickey
    // (u64 = 64 bits entero sin firmar = 8 bytes)
    #[account(init, payer = authority, space = 8 + 8 + 32)]
    pub counter: Account<'info, Counter>,
    #[account(mut)] //authority va a ser mutable (puede cambiar)
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Delete<'info>{
    #[account(mut)]
     pub authority: Signer<'info>,
     #[account(mut,
        constraint = counter.authority == counter.key(),
        close = authority
     )]
     pub counter: Account<'info, Counter>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut,
        constraint = counter.authority == authority.key(),
    )]
    pub counter: Account<'info, Counter>,
}

#[account] //lo convierte en una cuenta
pub struct Counter {
    number: u64, //64 bytes
    authority: Pubkey, //32 bytes
}

#[error_code]
pub enum ErrorCode {
    #[msg("You are not the authorized.")]
    NotAuthorized,
}