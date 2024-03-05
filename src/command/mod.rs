pub struct Data {}

pub type Error = Box<dyn std::error::Error + Sync + Send>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub mod math;
pub mod meta;
