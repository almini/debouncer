#![no_std]

pub struct Debouncer {
    state: bool,
    integrator: u8,
    limit: u8,
}

#[derive(Debug, PartialEq)]
pub enum DebouncerResult {
    State(bool),
    Transition(bool),
}

impl Debouncer {
    pub fn new(limit: u8) -> Debouncer {
        Debouncer {
            state: false,
            integrator: 0,
            limit,
        }
    }

    pub fn reset(&mut self) {
        self.state = false;
        self.integrator = 0;
    }

    pub fn update(&mut self, pressed: bool) -> DebouncerResult {
        if !pressed {
            self.integrator = self.integrator.saturating_sub(1);
        } else if self.integrator < self.limit {
            self.integrator += 1;
        }

        let old_state = self.state;
        if self.integrator == 0 {
            self.state = false;
        } else if self.integrator >= self.limit {
            self.state = true;
        }

        if self.state != old_state {
            DebouncerResult::Transition(self.state)
        } else {
            DebouncerResult::State(self.state)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut d = Debouncer::new(3);
        assert_eq!(d.update(true), DebouncerResult::State(false));
        assert_eq!(d.update(true), DebouncerResult::State(false));
        assert_eq!(d.update(true), DebouncerResult::Transition(true));
        assert_eq!(d.update(false), DebouncerResult::State(true));
        assert_eq!(d.update(false), DebouncerResult::State(true));
        assert_eq!(d.update(false), DebouncerResult::Transition(false));
    }
}
