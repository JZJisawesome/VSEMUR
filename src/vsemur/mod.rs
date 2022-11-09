
//TODO other functions

//Thanks https://stackoverflow.com/questions/67307526/is-it-possible-to-get-the-cargo-environment-variable-information-for-one-crate-u
//pub use vsemur::version;
pub mod about;
mod lifetime;

pub struct Registers {
    //TODO
}

pub struct State {
    num_ticks: u128,
    regs: Registers,

}

impl State {
    pub fn new(/*todo args*/) -> State {
        return State {
            num_ticks: 0,
            regs: Registers{},//TODO
        };
    }
}
