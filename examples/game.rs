#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_stm32 as _;
use mario::{AliveStates, BigMarioStates, Mario, MarioConsummables, States};
use panic_halt as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut mario = Mario::new(States::AliveMario(AliveStates::SmallMario));

    info!("Start");

    // Initial state
    assert!(mario.get_current_state() == States::AliveMario(AliveStates::SmallMario));
    assert!(mario.get_number_of_lifes() == 1);
    assert!(mario.get_number_of_coins() == 0);
    assert!(mario.is_alive());
    info!(
        "\n\tState: {}\n\tAlive: {}\n\tLifes: {}\n\tCoins: {}",
        mario.get_current_state(),
        mario.is_alive(),
        mario.get_number_of_lifes(),
        mario.get_number_of_coins()
    );

    // Get a mushroom
    mario.get_consummable(MarioConsummables::Mushroom);
    assert!(
        mario.get_current_state()
            == States::AliveMario(AliveStates::BigMario(BigMarioStates::SuperMario))
    );
    assert!(mario.get_number_of_lifes() == 1);
    assert!(mario.get_number_of_coins() == 100);
    assert!(mario.is_alive());
    info!(
        "\n\tState: {}\n\tAlive: {}\n\tLifes: {}\n\tCoins: {}",
        mario.get_current_state(),
        mario.is_alive(),
        mario.get_number_of_lifes(),
        mario.get_number_of_coins()
    );

    // Get a hit
    mario.get_hit();
    assert!(mario.get_current_state() == States::AliveMario(AliveStates::SmallMario));
    assert!(mario.get_number_of_lifes() == 1);
    assert!(mario.get_number_of_coins() == 0);
    assert!(mario.is_alive());
    info!(
        "\n\tState: {}\n\tAlive: {}\n\tLifes: {}\n\tCoins: {}",
        mario.get_current_state(),
        mario.is_alive(),
        mario.get_number_of_lifes(),
        mario.get_number_of_coins()
    );

    // Get a flower
    mario.get_consummable(MarioConsummables::Flower);
    assert!(
        mario.get_current_state()
            == States::AliveMario(AliveStates::BigMario(BigMarioStates::FireMario))
    );
    assert!(mario.get_number_of_lifes() == 1);
    assert!(mario.get_number_of_coins() == 200);
    assert!(mario.is_alive());
    info!(
        "\n\tState: {}\n\tAlive: {}\n\tLifes: {}\n\tCoins: {}",
        mario.get_current_state(),
        mario.is_alive(),
        mario.get_number_of_lifes(),
        mario.get_number_of_coins()
    );

    // Get a feather
    mario.get_consummable(MarioConsummables::Feather);
    assert!(
        mario.get_current_state()
            == States::AliveMario(AliveStates::BigMario(BigMarioStates::CapeMario))
    );
    assert!(mario.get_number_of_lifes() == 1);
    assert!(mario.get_number_of_coins() == 500);
    assert!(mario.is_alive());
    info!(
        "\n\tState: {}\n\tAlive: {}\n\tLifes: {}\n\tCoins: {}",
        mario.get_current_state(),
        mario.is_alive(),
        mario.get_number_of_lifes(),
        mario.get_number_of_coins()
    );

    // Get a hit
    mario.get_hit();
    assert!(mario.get_current_state() == States::AliveMario(AliveStates::SmallMario));
    assert!(mario.get_number_of_lifes() == 1);
    assert!(mario.get_number_of_coins() == 0);
    assert!(mario.is_alive());
    info!(
        "\n\tState: {}\n\tAlive: {}\n\tLifes: {}\n\tCoins: {}",
        mario.get_current_state(),
        mario.is_alive(),
        mario.get_number_of_lifes(),
        mario.get_number_of_coins()
    );

    // Oh no
    mario.get_hit();
    assert!(mario.get_current_state() == States::DeadMario);
    assert!(mario.get_number_of_lifes() == 0);
    assert!(mario.get_number_of_coins() == 0);
    assert!(!mario.is_alive());
    info!(
        "\n\tState: {}\n\tAlive: {}\n\tLifes: {}\n\tCoins: {}",
        mario.get_current_state(),
        mario.is_alive(),
        mario.get_number_of_lifes(),
        mario.get_number_of_coins()
    );

    info!("End");

    loop {
        cortex_m::asm::nop();
    }
}
