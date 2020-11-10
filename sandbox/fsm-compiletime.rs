// https://hoverbear.org/blog/rust-state-machine-pattern/
// https://play.rust-lang.org/?gist=ee3e4df093c136ced7b394dc7ffb78e1&version=stable&backtrace=0
// https://blog.yoshuawuyts.com/state-machines/#state-machines-in-rust-today

// is `Event<E, Next>` better?
trait Event<E> {
    type Next;
    fn next(self, event: E) -> Self::Next;
}

#[derive(Debug)]
struct State<Next> {
    _inner: Next,
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

impl State<states::Green> {
    pub fn new() -> State<states::Green> {
        State {
            _inner: states::Green {},
        }
    }
}

impl Event<events::GreenToYellow> for State<states::Green> {
    type Next = State<states::Yellow>;
    fn next(self, _event: events::GreenToYellow) -> Self::Next {
        State {
            _inner: states::Yellow {},
        }
    }
}

impl Event<events::YellowToGreen> for State<states::Yellow> {
    type Next = State<states::Green>;
    fn next(self, _event: events::YellowToGreen) -> Self::Next {
        State {
            _inner: states::Green {},
        }
    }
}

impl Event<events::YellowToRed> for State<states::Yellow> {
    type Next = State<states::Red>;
    fn next(self, _event: events::YellowToRed) -> Self::Next {
        State {
            _inner: states::Red {},
        }
    }
}

impl Event<events::RedToGreen> for State<states::Red> {
    type Next = State<states::Green>;
    fn next(self, _event: events::RedToGreen) -> Self::Next {
        State {
            _inner: states::Green {},
        }
    }
}

fn main() {
    let state = State::new(); // green
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
    // >   error[E0277]: the trait bound `State<Yellow>: Event<RedToGreen>` is not satisfied
    // >   --> sandbox/fsm-compiletime.rs:99:23
    // >    |
    // > 99 |     let state = state.next(events::RedToGreen);
    // >    |                       ^^^^ the trait `Event<RedToGreen>` is not implemented for `State<Yellow>`
    // >    |
    // >    = help: the following implementations were found:
    // >              <State<Green> as Event<GreenToYellow>>
    // >              <State<Red> as Event<RedToGreen>>
    // >              <State<Yellow> as Event<YellowToGreen>>
    // >              <State<Yellow> as Event<YellowToRed>>
    // tells you what state you were in and what event occurred: `State<Yellow>: Event<RedToGreen>`
    // tells you what events are available and their starting states, but not what they transition into, e.g. `<State<Green> as Event<GreenToYellow>>`
}
