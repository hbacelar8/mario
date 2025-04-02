#![no_std]
#![no_main]

#[cfg(feature = "trace")]
use defmt::info;

#[cfg(feature = "trace")]
use defmt_rtt as _;

use embassy_stm32 as _;
use mario::{AliveStates, BigMarioStates, Mario, MarioConsummables, States};
use panic_halt as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut mario = Mario::new(States::AliveMario(AliveStates::SmallMario));

    #[cfg(feature = "trace")]
    info!("Start");

    // Initial state
    assert!(mario.current_state() == States::AliveMario(AliveStates::SmallMario));
    assert!(mario.number_of_lifes() == 1);
    assert!(mario.number_of_coins() == 0);
    assert!(mario.is_alive());
    #[cfg(feature = "trace")]
    info!(
        "\n\tState: {}\n\tAlive: {}\n\tLifes: {}\n\tCoins: {}",
        mario.current_state(),
        mario.is_alive(),
        mario.number_of_lifes(),
        mario.number_of_coins()
    );

    // Get a mushroom
    mario.get_consummable(MarioConsummables::Mushroom);
    assert!(
        mario.current_state()
            == States::AliveMario(AliveStates::BigMario(BigMarioStates::SuperMario))
    );
    assert!(mario.number_of_lifes() == 1);
    assert!(mario.number_of_coins() == 100);
    assert!(mario.is_alive());
    #[cfg(feature = "trace")]
    info!(
        "\n\tState: {}\n\tAlive: {}\n\tLifes: {}\n\tCoins: {}",
        mario.current_state(),
        mario.is_alive(),
        mario.number_of_lifes(),
        mario.number_of_coins()
    );

    // Get a hit
    mario.get_hit();
    assert!(mario.current_state() == States::AliveMario(AliveStates::SmallMario));
    assert!(mario.number_of_lifes() == 1);
    assert!(mario.number_of_coins() == 0);
    assert!(mario.is_alive());
    #[cfg(feature = "trace")]
    info!(
        "\n\tState: {}\n\tAlive: {}\n\tLifes: {}\n\tCoins: {}",
        mario.current_state(),
        mario.is_alive(),
        mario.number_of_lifes(),
        mario.number_of_coins()
    );

    // Get a flower
    mario.get_consummable(MarioConsummables::Flower);
    assert!(
        mario.current_state()
            == States::AliveMario(AliveStates::BigMario(BigMarioStates::FireMario))
    );
    assert!(mario.number_of_lifes() == 1);
    assert!(mario.number_of_coins() == 200);
    assert!(mario.is_alive());
    #[cfg(feature = "trace")]
    info!(
        "\n\tState: {}\n\tAlive: {}\n\tLifes: {}\n\tCoins: {}",
        mario.current_state(),
        mario.is_alive(),
        mario.number_of_lifes(),
        mario.number_of_coins()
    );

    // Get a feather
    mario.get_consummable(MarioConsummables::Feather);
    assert!(
        mario.current_state()
            == States::AliveMario(AliveStates::BigMario(BigMarioStates::CapeMario))
    );
    assert!(mario.number_of_lifes() == 1);
    assert!(mario.number_of_coins() == 500);
    assert!(mario.is_alive());
    #[cfg(feature = "trace")]
    info!(
        "\n\tState: {}\n\tAlive: {}\n\tLifes: {}\n\tCoins: {}",
        mario.current_state(),
        mario.is_alive(),
        mario.number_of_lifes(),
        mario.number_of_coins()
    );

    // Some more feathers until +1 life
    mario.get_consummable(MarioConsummables::Feather);
    mario.get_consummable(MarioConsummables::Feather);
    assert!(
        mario.current_state()
            == States::AliveMario(AliveStates::BigMario(BigMarioStates::CapeMario))
    );
    assert!(mario.number_of_lifes() == 2);
    assert!(mario.number_of_coins() == 0);
    assert!(mario.is_alive());
    #[cfg(feature = "trace")]
    info!(
        "\n\tState: {}\n\tAlive: {}\n\tLifes: {}\n\tCoins: {}",
        mario.current_state(),
        mario.is_alive(),
        mario.number_of_lifes(),
        mario.number_of_coins()
    );

    // Get a hit
    mario.get_hit();
    assert!(mario.current_state() == States::AliveMario(AliveStates::SmallMario));
    assert!(mario.number_of_lifes() == 2);
    assert!(mario.number_of_coins() == 0);
    assert!(mario.is_alive());
    #[cfg(feature = "trace")]
    info!(
        "\n\tState: {}\n\tAlive: {}\n\tLifes: {}\n\tCoins: {}",
        mario.current_state(),
        mario.is_alive(),
        mario.number_of_lifes(),
        mario.number_of_coins()
    );

    // Another one
    mario.get_hit();
    assert!(mario.current_state() == States::AliveMario(AliveStates::SmallMario));
    assert!(mario.number_of_lifes() == 1);
    assert!(mario.number_of_coins() == 0);
    assert!(mario.is_alive());
    #[cfg(feature = "trace")]
    info!(
        "\n\tState: {}\n\tAlive: {}\n\tLifes: {}\n\tCoins: {}",
        mario.current_state(),
        mario.is_alive(),
        mario.number_of_lifes(),
        mario.number_of_coins()
    );

    // Oh no
    mario.get_hit();
    assert!(mario.current_state() == States::DeadMario);
    assert!(mario.number_of_lifes() == 0);
    assert!(mario.number_of_coins() == 0);
    assert!(!mario.is_alive());
    #[cfg(feature = "trace")]
    info!(
        "\n\tState: {}\n\tAlive: {}\n\tLifes: {}\n\tCoins: {}",
        mario.current_state(),
        mario.is_alive(),
        mario.number_of_lifes(),
        mario.number_of_coins()
    );

    #[cfg(feature = "trace")]
    info!("End");

    loop {
        cortex_m::asm::nop();
    }
}
