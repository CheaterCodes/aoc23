use std::{num::NonZeroU32, time::{Instant, Duration}};

#[derive(Clone, Copy)]
enum State {
    Init,
    O,
    On,
    T,
    Tw,
    Th,
    Thr,
    Thre,
    F,
    Fo,
    Fou,
    Fi,
    Fiv,
    S,
    Si,
    Se,
    Sev,
    Seve,
    E,
    Ei,
    Eig,
    Eigh,
    N,
    Ni,
    Nin,
}

fn main() {
    let input = std::fs::read_to_string("./input/day1.txt").unwrap();

    let initial = Instant::now();

    while initial.elapsed() < Duration::from_millis(100) {
        let start = Instant::now();

        let sum = with_tables(&input);

        let elapsed = start.elapsed();
        println!("Finished: {sum}, time: {}", elapsed.as_micros());
    }

}

fn fast(input: &str) -> u32 {
    let mut state = State::Init;
    let mut first = None;
    let mut last = None;
    let mut sum = 0;

    for byte in input.bytes() {
        if let Some(d) = state.advance(byte) {
            first.get_or_insert(d);
            last = Some(d);
        }

        if byte == b'\n' {
            sum += first.unwrap().get() * 10 + last.unwrap().get();
            first = None;
        }
    }

    sum
}

fn get_transition_table() -> [[State; 256]; 25] {
    let mut table = [[State::Init; 256]; 25];

    // Initialize defaults
    for state in 0..table.len() {
        table[state][b'1' as usize] = State::Init;
        table[state][b'2' as usize] = State::Init;
        table[state][b'3' as usize] = State::Init;
        table[state][b'4' as usize] = State::Init;
        table[state][b'5' as usize] = State::Init;
        table[state][b'6' as usize] = State::Init;
        table[state][b'7' as usize] = State::Init;
        table[state][b'8' as usize] = State::Init;
        table[state][b'9' as usize] = State::Init;

        table[state][b'e' as usize] = State::E;
        table[state][b'n' as usize] = State::N;
        table[state][b's' as usize] = State::S;
        table[state][b'f' as usize] = State::F;
        table[state][b't' as usize] = State::T;
        table[state][b'o' as usize] = State::O;
    }

    table[State::O    as usize][b'n' as usize] = State::On;
    table[State::Fo   as usize][b'n' as usize] = State::On;
    table[State::On   as usize][b'e' as usize] = State::E;
    table[State::T    as usize][b'w' as usize] = State::Tw;
    table[State::Tw   as usize][b'o' as usize] = State::O;
    table[State::T    as usize][b'h' as usize] = State::Th;
    table[State::Th   as usize][b'r' as usize] = State::Thr;
    table[State::Thr  as usize][b'e' as usize] = State::Thre;
    table[State::Thre as usize][b'e' as usize] = State::E;
    table[State::Init as usize][b'f' as usize] = State::F;
    table[State::F    as usize][b'o' as usize] = State::Fo;
    table[State::Fo   as usize][b'u' as usize] = State::Fou;
    table[State::Fou  as usize][b'r' as usize] = State::Init;
    table[State::F    as usize][b'i' as usize] = State::Fi;
    table[State::Fi   as usize][b'v' as usize] = State::Fiv;
    table[State::Fiv  as usize][b'e' as usize] = State::E;
    table[State::S    as usize][b'i' as usize] = State::Si;
    table[State::Si   as usize][b'x' as usize] = State::Init;
    table[State::S    as usize][b'e' as usize] = State::Se;
    table[State::Se   as usize][b'v' as usize] = State::Sev;
    table[State::Sev  as usize][b'e' as usize] = State::Seve;
    table[State::Seve as usize][b'n' as usize] = State::N;
    table[State::E    as usize][b'i' as usize] = State::Ei;
    table[State::Thre as usize][b'i' as usize] = State::Ei;
    table[State::Fiv  as usize][b'i' as usize] = State::Ei;
    table[State::Se   as usize][b'i' as usize] = State::Ei;
    table[State::Seve as usize][b'i' as usize] = State::Ei;
    table[State::Ei   as usize][b'g' as usize] = State::Eig;
    table[State::Eig  as usize][b'h' as usize] = State::Eigh;
    table[State::Eigh as usize][b't' as usize] = State::T;
    table[State::N    as usize][b'i' as usize] = State::Ni;
    table[State::Nin  as usize][b'i' as usize] = State::Ni;
    table[State::On   as usize][b'i' as usize] = State::Ni;
    table[State::Ni   as usize][b'n' as usize] = State::Nin;
    table[State::Nin  as usize][b'e' as usize] = State::E;

    table
}

fn get_action_table() -> [[Option<NonZeroU32>; 256]; 25] {
    let mut table = [[None; 256]; 25];

    // Initialize defaults
    for state in 0..table.len() {
        table[state][b'1' as usize] = NonZeroU32::new(1);
        table[state][b'2' as usize] = NonZeroU32::new(2);
        table[state][b'3' as usize] = NonZeroU32::new(3);
        table[state][b'4' as usize] = NonZeroU32::new(4);
        table[state][b'5' as usize] = NonZeroU32::new(5);
        table[state][b'6' as usize] = NonZeroU32::new(6);
        table[state][b'7' as usize] = NonZeroU32::new(7);
        table[state][b'8' as usize] = NonZeroU32::new(8);
        table[state][b'9' as usize] = NonZeroU32::new(9);
    }

    table[State::On   as usize][b'e' as usize] = NonZeroU32::new(1);
    table[State::Tw   as usize][b'o' as usize] = NonZeroU32::new(2);
    table[State::Thre as usize][b'e' as usize] = NonZeroU32::new(3);
    table[State::Fou  as usize][b'r' as usize] = NonZeroU32::new(4);
    table[State::Fiv  as usize][b'e' as usize] = NonZeroU32::new(5);
    table[State::Si   as usize][b'x' as usize] = NonZeroU32::new(6);
    table[State::Seve as usize][b'n' as usize] = NonZeroU32::new(7);
    table[State::Eigh as usize][b't' as usize] = NonZeroU32::new(8);
    table[State::Nin  as usize][b'e' as usize] = NonZeroU32::new(9);

    table
}

fn with_tables(input: &str) -> u32 {
    let mut state = State::Init;

    let next = get_transition_table();
    let result = get_action_table();

    let mut first = None;
    let mut last = None;
    let mut sum = 0;

    for byte in input.bytes() {
        if let Some(d) = result[state as usize][byte as usize] {
            first.get_or_insert(d);
            last = Some(d);
        }

        state = next[state as usize][byte as usize];

        if byte == b'\n' {
            sum += first.unwrap().get() * 10 + last.unwrap().get();
            first = None;
        }
    }

    sum
}
