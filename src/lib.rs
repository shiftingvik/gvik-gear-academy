#![no_std]
use gstd::{exec, msg, prelude::*};
use pebbles_game_io::*;

static mut GAME_STATE: Option<GameState> = None;


fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

#[no_mangle]
extern "C" fn init() {

}

#[no_mangle]
extern "C" fn handle() {

}



#[no_mangle]
extern "C" fn state() {
    msg::reply(get_state().clone(), 0).expect("Failed to share state");
}

fn get_state_mut() -> &'static mut GameState {
    unsafe { GAME_STATE.as_mut().expect("Game state is not initialized") }
}

fn get_state() -> &'static GameState {
    unsafe { GAME_STATE.as_ref().expect("Game state is not initialized") }
}
