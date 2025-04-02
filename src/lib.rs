#![no_std]

use defmt::Format;
use rustfsm::{StateBehavior, rustfsm};

#[derive(Clone, Copy, PartialEq)]
pub enum MarioConsummables {
    Mushroom,
    Flower,
    Feather,
}

#[derive(Clone, Copy, PartialEq, Format)]
pub enum AliveStates {
    SmallMario,
    BigMario(BigMarioStates),
}

#[derive(Clone, Copy, PartialEq, Format)]
pub enum BigMarioStates {
    SuperMario,
    CapeMario,
    FireMario,
}

#[derive(Clone, Copy, PartialEq, Format)]
pub enum States {
    AliveMario(AliveStates),
    DeadMario,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Events {
    GetConsummables(MarioConsummables),
    GetHit,
}

pub struct Context {
    number_of_lifes: u8,
    number_of_coins: u16,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            number_of_lifes: 1,
            number_of_coins: 0,
        }
    }
}

// Generate the state machine
rustfsm!(Mario, States, Events, Context);

impl StateBehavior for States {
    type State = States;
    type Event = Events;
    type Context = Context;

    fn enter(&self, context: &mut Self::Context) {
        match self {
            States::AliveMario(AliveStates::SmallMario) => context.number_of_coins = 0,

            States::AliveMario(AliveStates::BigMario(BigMarioStates::SuperMario)) => {
                context.number_of_coins += 100
            }

            States::AliveMario(AliveStates::BigMario(BigMarioStates::FireMario)) => {
                context.number_of_coins += 200
            }

            States::AliveMario(AliveStates::BigMario(BigMarioStates::CapeMario)) => {
                context.number_of_coins += 300
            }

            States::DeadMario => context.number_of_lifes = 0,
        }

        if context.number_of_coins >= 1000 {
            context.number_of_coins = 0;
            context.number_of_lifes += 1;
        }
    }

    fn handle(&self, event: &Self::Event, context: &mut Self::Context) -> Option<Self::State> {
        use AliveStates::*;
        use BigMarioStates::*;
        use Events::*;
        use MarioConsummables::*;
        use States::*;

        match (self, event) {
            (AliveMario(SmallMario), GetConsummables(Mushroom)) => {
                Some(AliveMario(BigMario(SuperMario)))
            }

            (
                AliveMario(SmallMario)
                | AliveMario(BigMario(SuperMario))
                | AliveMario(BigMario(CapeMario)),
                GetConsummables(Flower),
            ) => Some(AliveMario(BigMario(FireMario))),

            (
                AliveMario(SmallMario)
                | AliveMario(BigMario(SuperMario))
                | AliveMario(BigMario(FireMario)),
                GetConsummables(Feather),
            ) => Some(AliveMario(BigMario(CapeMario))),

            (AliveMario(SmallMario), GetHit) => {
                context.number_of_lifes = 0;
                Some(DeadMario)
            }

            (AliveMario(BigMario(_)), GetHit) => {
                if context.number_of_lifes == 1 {
                    Some(AliveMario(SmallMario))
                } else {
                    context.number_of_lifes -= 1;
                    None
                }
            }

            _ => None,
        }
    }
}

impl Mario {
    pub fn is_alive(&self) -> bool {
        self.current_state != States::DeadMario
    }

    pub fn get_number_of_lifes(&self) -> u8 {
        self.context.number_of_lifes
    }

    pub fn get_number_of_coins(&self) -> u16 {
        self.context.number_of_coins
    }

    pub fn get_consummable(&mut self, consummable: MarioConsummables) {
        self.handle(Events::GetConsummables(consummable));
    }

    pub fn get_hit(&mut self) {
        self.handle(Events::GetHit);
    }
}
