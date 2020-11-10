// https://hoverbear.org/blog/rust-state-machine-pattern/
// https://play.rust-lang.org/?gist=ee3e4df093c136ced7b394dc7ffb78e1&version=stable&backtrace=0
// https://blog.yoshuawuyts.com/state-machines/#state-machines-in-rust-today

trait Event<E, Next> {
    fn next(self, event: E) -> Next;
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

impl Event<events::GreenToYellow, State<states::Yellow>> for State<states::Green> {
    fn next(self, _event: events::GreenToYellow) -> State<states::Yellow> {
        State {
            _inner: states::Yellow {},
        }
    }
}

impl Event<events::YellowToGreen, State<states::Green>> for State<states::Yellow> {
    fn next(self, _event: events::YellowToGreen) -> State<states::Green> {
        State {
            _inner: states::Green {},
        }
    }
}

impl Event<events::YellowToRed, State<states::Red>> for State<states::Yellow> {
    fn next(self, _event: events::YellowToRed) -> State<states::Red> {
        State {
            _inner: states::Red {},
        }
    }
}

impl Event<events::RedToGreen, State<states::Green>> for State<states::Red> {
    fn next(self, _event: events::RedToGreen) -> State<states::Green> {
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
    // >   error[E0277]: the trait bound `State<Yellow>: Event<RedToGreen, _>` is not satisfied
    // >   --> sandbox/fsm-compiletime2.rs:94:23
    // >    |
    // > 94 |     let state = state.next(events::RedToGreen);
    // >    |                       ^^^^ the trait `Event<RedToGreen, _>` is not implemented for `State<Yellow>`
    // >    |
    // >    = help: the following implementations were found:
    // >              <State<Green> as Event<GreenToYellow, State<Yellow>>>
    // >              <State<Red> as Event<RedToGreen, State<Green>>>
    // >              <State<Yellow> as Event<YellowToGreen, State<Green>>>
    // >              <State<Yellow> as Event<YellowToRed, State<Red>>>
    // tells you what state you were in and what event occurred: `State<Yellow>: Event<RedToGreen, _>`
    // tells you what state transitions and corresponding events are available e.g. `<State<Green> as Event<GreenToYellow, State<Yellow>>>`
}
