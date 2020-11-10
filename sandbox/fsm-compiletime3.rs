// https://hoverbear.org/blog/rust-state-machine-pattern/
// https://play.rust-lang.org/?gist=ee3e4df093c136ced7b394dc7ffb78e1&version=stable&backtrace=0
// https://blog.yoshuawuyts.com/state-machines/#state-machines-in-rust-today

trait Event<E, Next> {
    fn next(self, event: E) -> Next;
}

// ad-hoc enum definition

mod states {
    #[derive(Debug)]
    pub struct Green;
    #[derive(Debug)]
    pub struct Yellow;
    #[derive(Debug)]
    pub struct Red;
}

mod events {
    #[derive(Debug)]
    pub struct GreenToYellow;
    #[derive(Debug)]
    pub struct YellowToGreen;
    #[derive(Debug)]
    pub struct YellowToRed;
    #[derive(Debug)]
    pub struct RedToGreen;
}

impl states::Green {
    pub fn new() -> states::Green {
        states::Green {}
    }
}

impl Event<events::GreenToYellow, states::Yellow> for states::Green {
    fn next(self, _event: events::GreenToYellow) -> states::Yellow {
        states::Yellow {}
    }
}

impl Event<events::YellowToGreen, states::Green> for states::Yellow {
    fn next(self, _event: events::YellowToGreen) -> states::Green {
        states::Green {}
    }
}

impl Event<events::YellowToRed, states::Red> for states::Yellow {
    fn next(self, _event: events::YellowToRed) -> states::Red {
        states::Red {}
    }
}

impl Event<events::RedToGreen, states::Green> for states::Red {
    fn next(self, _event: events::RedToGreen) -> states::Green {
        states::Green {}
    }
}

fn main() {
    let state = states::Green::new(); // green
    dbg!(&state);
    let state = state.next(events::GreenToYellow); // green -> yellow
    dbg!(&state);
    let state = state.next(events::YellowToRed); // yellow -> red
    dbg!(&state);
    let state = state.next(events::RedToGreen); // red -> green
    dbg!(&state);
    let state = state.next(events::GreenToYellow); // green -> yellow
    dbg!(&state);
    let state = state.next(events::YellowToGreen); // yellow -> green
    dbg!(&state);
    let state = state.next(events::GreenToYellow); // green -> yellow
    dbg!(&state);

    // uncommenting the below (

    // let state = state.next(events::RedToGreen);
    // dbg!(&state);

    // ) will lead to a compile time error:
    // >   error[E0277]: the trait bound `Yellow: Event<RedToGreen, _>` is not satisfied
    // >   --> sandbox/fsm-compiletime3.rs:79:23
    // >    |
    // > 79 |     let state = state.next(events::RedToGreen);
    // >    |                       ^^^^ the trait `Event<RedToGreen, _>` is not implemented for `Yellow`
    // >    |
    // >    = help: the following implementations were found:
    // >              <Yellow as Event<YellowToGreen, Green>>
    // >              <Yellow as Event<YellowToRed, Red>>
    // tells you what state you were in and what event occurred: `Yellow: Event<RedToGreen, _>`
    // tells you what state transitions and corresponding events are available, but only for that particular state e.g. `<Yellow as Event<YellowToGreen, Green>>`
}

// next steps:
// 1. turn State into an enum that wraps each unique type: `enum States { Green(states::Green) }`
// this makes State enumerable
// 2. create an Events enum that wraps each unique type: `enum Events { GreenToYellow(events::GreenToYellow) }`
// this makes Events enumerable
// 3. we could store data in an event like `mod events { pub struct GreenToYellow(f32); }`, as well as in a state like `mod states { pub struct Green(String); }
// you can use the data in an event (message) to inform the new state, or update a shared data store
// https://www.gamasutra.com/blogs/ChrisSimpson/20140717/221339/Behavior_trees_for_AI_How_they_work.php
// https://web.archive.org/web/20150214085327/http://guineashots.com/2014/07/25/an-introduction-to-behavior-trees-part-1/
// https://takinginitiative.wordpress.com/2014/02/17/synchronized-behavior-trees/