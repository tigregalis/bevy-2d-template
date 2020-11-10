// https://hoverbear.org/blog/rust-state-machine-pattern/
// https://play.rust-lang.org/?gist=ee3e4df093c136ced7b394dc7ffb78e1&version=stable&backtrace=0
// https://blog.yoshuawuyts.com/state-machines/#state-machines-in-rust-today

#[derive(Debug, PartialEq)]
enum State {
    Waiting { waiting_time: usize },
    Filling { rate: usize },
    Done,
    Failure(String),
}

#[derive(Debug, Clone, Copy)]
enum Event {
    NothingHappend,
    InsertBottle,
    BottleFull,
    BottleEjected,
}

impl State {
    fn next(self, event: Event) -> State {
        match (self, event) {
            (State::Waiting { waiting_time }, Event::NothingHappend) => State::Waiting {
                waiting_time: waiting_time + 1,
            },
            (State::Waiting { .. }, Event::InsertBottle) => State::Filling { rate: 10 },
            (State::Filling { rate }, Event::BottleFull) => State::Done,
            (State::Done, Event::BottleEjected) => State::Waiting { waiting_time: 0 },
            (s, e) => State::Failure(
                format!("Wrong state, event combination: {:#?} {:#?}", s, e).to_string(),
            ),
        }
    }
    fn run(&self) {
        match *self {
            State::Waiting { waiting_time } => {
                println!("We waited for {}", waiting_time);
            }
            State::Filling { rate } => {
                // put stuff in bottle at rate 'rate'
            }
            State::Done | State::Failure(_) => {}
        }
    }
}

fn main() {
    let mut state = State::Waiting { waiting_time: 0 };

    // Sequence of events (might be dynamical based on what State::run did)
    let events = [
        Event::NothingHappend,
        Event::NothingHappend,
        Event::InsertBottle,
        Event::BottleFull,
        Event::BottleEjected,
        Event::NothingHappend,
        Event::BottleFull,
    ];
    let mut iter = events.iter();

    loop {
        // just a hack to get owned values, because I used an iterator
        let event = iter.next().unwrap().clone();
        print!("__ Transition from {:?}", state);
        state = state.next(event);
        println!(" to {:?}", state);

        if let State::Failure(string) = state {
            println!("{}", string);
            break;
        } else {
            // You might want to do somethin while in a state
            // You could also add State::enter() and State::exit()
            state.run();
        }
    }
}

// other examples
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=13d7f2a98003560e4a53a2d2a3b740e5
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=2a978a9867b9a6b01660249cb6c64f80
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=addf56e901fa2500acc9e5018c7d1429
// this thread https://www.reddit.com/r/rust/comments/ft1hqh/state_machines_in_rust/
// this thread https://www.reddit.com/r/rust/comments/9vkbb2/what_do_you_guys_use_for_coding_state_machines/
