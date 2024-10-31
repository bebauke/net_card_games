pub mod hosn_obi;
pub mod schnapsen;
pub mod skat;
pub trait Game {
    fn start_game(&mut self);
    fn status(&self) -> String; // Status as a String
    fn reset(&mut self);
}

