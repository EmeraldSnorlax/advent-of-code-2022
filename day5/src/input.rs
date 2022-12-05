pub type Crate = char;
pub type Stack = Vec<Crate>;
pub type Yard = [Stack; 9];

#[derive(Debug)]
pub struct Operation {
    pub from: u8,
    pub to: u8,
    pub count: u8,
}

// Vertical parsing is cruel. I want to have a polite peaceful negotiation using a tire iron for whoever was responsible for this.
pub fn get() -> (Yard, [Operation; 502]) {
    let mut operations: [Operation; 502] = [
        Operation {
            count: 2,
            from: 4,
            to: 6,
        },
        Operation {
            count: 1,
            from: 9,
            to: 5,
        },
        Operation {
            count: 3,
            from: 2,
            to: 4,
        },
        Operation {
            count: 8,
            from: 4,
            to: 7,
        },
        Operation {
            count: 2,
            from: 9,
            to: 7,
        },
        Operation {
            count: 3,
            from: 8,
            to: 3,
        },
        Operation {
            count: 2,
            from: 1,
            to: 2,
        },
        Operation {
            count: 5,
            from: 7,
            to: 9,
        },
        Operation {
            count: 1,
            from: 9,
            to: 4,
        },
        Operation {
            count: 1,
            from: 8,
            to: 3,
        },
        Operation {
            count: 1,
            from: 3,
            to: 4,
        },
        Operation {
            count: 2,
            from: 4,
            to: 9,
        },
        Operation {
            count: 7,
            from: 3,
            to: 5,
        },
        Operation {
            count: 6,
            from: 1,
            to: 8,
        },
        Operation {
            count: 11,
            from: 7,
            to: 9,
        },
        Operation {
            count: 12,
            from: 5,
            to: 3,
        },
        Operation {
            count: 6,
            from: 6,
            to: 9,
        },
        Operation {
            count: 3,
            from: 3,
            to: 8,
        },
        Operation {
            count: 4,
            from: 2,
            to: 7,
        },
        Operation {
            count: 3,
            from: 5,
            to: 7,
        },
        Operation {
            count: 1,
            from: 5,
            to: 7,
        },
        Operation {
            count: 2,
            from: 2,
            to: 5,
        },
        Operation {
            count: 1,
            from: 5,
            to: 2,
        },
        Operation {
            count: 5,
            from: 8,
            to: 9,
        },
        Operation {
            count: 7,
            from: 7,
            to: 2,
        },
        Operation {
            count: 3,
            from: 8,
            to: 7,
        },
        Operation {
            count: 1,
            from: 8,
            to: 9,
        },
        Operation {
            count: 4,
            from: 3,
            to: 6,
        },
        Operation {
            count: 1,
            from: 5,
            to: 1,
        },
        Operation {
            count: 9,
            from: 9,
            to: 6,
        },
        Operation {
            count: 7,
            from: 9,
            to: 6,
        },
        Operation {
            count: 20,
            from: 6,
            to: 5,
        },
        Operation {
            count: 12,
            from: 9,
            to: 8,
        },
        Operation {
            count: 5,
            from: 5,
            to: 1,
        },
        Operation {
            count: 3,
            from: 7,
            to: 4,
        },
        Operation {
            count: 6,
            from: 2,
            to: 7,
        },
        Operation {
            count: 2,
            from: 3,
            to: 1,
        },
        Operation {
            count: 4,
            from: 3,
            to: 8,
        },
        Operation {
            count: 1,
            from: 4,
            to: 1,
        },
        Operation {
            count: 7,
            from: 7,
            to: 5,
        },
        Operation {
            count: 4,
            from: 8,
            to: 2,
        },
        Operation {
            count: 3,
            from: 6,
            to: 2,
        },
        Operation {
            count: 3,
            from: 2,
            to: 9,
        },
        Operation {
            count: 4,
            from: 1,
            to: 7,
        },
        Operation {
            count: 2,
            from: 1,
            to: 2,
        },
        Operation {
            count: 3,
            from: 9,
            to: 5,
        },
        Operation {
            count: 11,
            from: 8,
            to: 5,
        },
        Operation {
            count: 1,
            from: 6,
            to: 9,
        },
        Operation {
            count: 1,
            from: 8,
            to: 5,
        },
        Operation {
            count: 1,
            from: 1,
            to: 2,
        },
        Operation {
            count: 24,
            from: 5,
            to: 4,
        },
        Operation {
            count: 2,
            from: 1,
            to: 6,
        },
        Operation {
            count: 11,
            from: 5,
            to: 4,
        },
        Operation {
            count: 2,
            from: 7,
            to: 9,
        },
        Operation {
            count: 1,
            from: 6,
            to: 2,
        },
        Operation {
            count: 4,
            from: 2,
            to: 1,
        },
        Operation {
            count: 28,
            from: 4,
            to: 2,
        },
        Operation {
            count: 1,
            from: 7,
            to: 8,
        },
        Operation {
            count: 9,
            from: 2,
            to: 5,
        },
        Operation {
            count: 2,
            from: 9,
            to: 6,
        },
        Operation {
            count: 4,
            from: 4,
            to: 2,
        },
        Operation {
            count: 1,
            from: 7,
            to: 4,
        },
        Operation {
            count: 3,
            from: 4,
            to: 7,
        },
        Operation {
            count: 1,
            from: 6,
            to: 9,
        },
        Operation {
            count: 21,
            from: 2,
            to: 3,
        },
        Operation {
            count: 3,
            from: 1,
            to: 6,
        },
        Operation {
            count: 5,
            from: 6,
            to: 2,
        },
        Operation {
            count: 7,
            from: 2,
            to: 3,
        },
        Operation {
            count: 1,
            from: 9,
            to: 3,
        },
        Operation {
            count: 1,
            from: 8,
            to: 4,
        },
        Operation {
            count: 1,
            from: 7,
            to: 8,
        },
        Operation {
            count: 3,
            from: 5,
            to: 8,
        },
        Operation {
            count: 1,
            from: 1,
            to: 7,
        },
        Operation {
            count: 2,
            from: 7,
            to: 9,
        },
        Operation {
            count: 2,
            from: 8,
            to: 4,
        },
        Operation {
            count: 1,
            from: 9,
            to: 2,
        },
        Operation {
            count: 1,
            from: 8,
            to: 6,
        },
        Operation {
            count: 11,
            from: 3,
            to: 4,
        },
        Operation {
            count: 1,
            from: 7,
            to: 8,
        },
        Operation {
            count: 6,
            from: 5,
            to: 9,
        },
        Operation {
            count: 2,
            from: 8,
            to: 7,
        },
        Operation {
            count: 1,
            from: 6,
            to: 5,
        },
        Operation {
            count: 7,
            from: 3,
            to: 8,
        },
        Operation {
            count: 9,
            from: 3,
            to: 6,
        },
        Operation {
            count: 1,
            from: 8,
            to: 3,
        },
        Operation {
            count: 1,
            from: 7,
            to: 4,
        },
        Operation {
            count: 2,
            from: 3,
            to: 5,
        },
        Operation {
            count: 4,
            from: 5,
            to: 7,
        },
        Operation {
            count: 4,
            from: 6,
            to: 8,
        },
        Operation {
            count: 2,
            from: 7,
            to: 9,
        },
        Operation {
            count: 11,
            from: 4,
            to: 2,
        },
        Operation {
            count: 1,
            from: 4,
            to: 2,
        },
        Operation {
            count: 6,
            from: 8,
            to: 9,
        },
        Operation {
            count: 1,
            from: 7,
            to: 1,
        },
        Operation {
            count: 1,
            from: 3,
            to: 7,
        },
        Operation {
            count: 3,
            from: 7,
            to: 8,
        },
        Operation {
            count: 6,
            from: 8,
            to: 9,
        },
        Operation {
            count: 6,
            from: 4,
            to: 8,
        },
        Operation {
            count: 18,
            from: 9,
            to: 3,
        },
        Operation {
            count: 1,
            from: 5,
            to: 8,
        },
        Operation {
            count: 5,
            from: 6,
            to: 5,
        },
        Operation {
            count: 6,
            from: 8,
            to: 1,
        },
        Operation {
            count: 3,
            from: 5,
            to: 4,
        },
        Operation {
            count: 1,
            from: 9,
            to: 8,
        },
        Operation {
            count: 3,
            from: 4,
            to: 8,
        },
        Operation {
            count: 15,
            from: 3,
            to: 6,
        },
        Operation {
            count: 2,
            from: 5,
            to: 9,
        },
        Operation {
            count: 3,
            from: 3,
            to: 1,
        },
        Operation {
            count: 9,
            from: 6,
            to: 4,
        },
        Operation {
            count: 2,
            from: 1,
            to: 5,
        },
        Operation {
            count: 2,
            from: 5,
            to: 8,
        },
        Operation {
            count: 6,
            from: 4,
            to: 2,
        },
        Operation {
            count: 6,
            from: 1,
            to: 6,
        },
        Operation {
            count: 3,
            from: 4,
            to: 6,
        },
        Operation {
            count: 6,
            from: 9,
            to: 1,
        },
        Operation {
            count: 4,
            from: 2,
            to: 1,
        },
        Operation {
            count: 7,
            from: 8,
            to: 1,
        },
        Operation {
            count: 1,
            from: 6,
            to: 7,
        },
        Operation {
            count: 17,
            from: 1,
            to: 5,
        },
        Operation {
            count: 1,
            from: 7,
            to: 1,
        },
        Operation {
            count: 5,
            from: 2,
            to: 1,
        },
        Operation {
            count: 1,
            from: 8,
            to: 6,
        },
        Operation {
            count: 11,
            from: 6,
            to: 4,
        },
        Operation {
            count: 2,
            from: 2,
            to: 3,
        },
        Operation {
            count: 3,
            from: 1,
            to: 8,
        },
        Operation {
            count: 7,
            from: 2,
            to: 5,
        },
        Operation {
            count: 4,
            from: 6,
            to: 7,
        },
        Operation {
            count: 4,
            from: 1,
            to: 5,
        },
        Operation {
            count: 15,
            from: 5,
            to: 9,
        },
        Operation {
            count: 2,
            from: 3,
            to: 7,
        },
        Operation {
            count: 2,
            from: 8,
            to: 2,
        },
        Operation {
            count: 1,
            from: 1,
            to: 9,
        },
        Operation {
            count: 6,
            from: 2,
            to: 6,
        },
        Operation {
            count: 7,
            from: 5,
            to: 6,
        },
        Operation {
            count: 5,
            from: 7,
            to: 3,
        },
        Operation {
            count: 1,
            from: 6,
            to: 1,
        },
        Operation {
            count: 2,
            from: 3,
            to: 4,
        },
        Operation {
            count: 1,
            from: 3,
            to: 4,
        },
        Operation {
            count: 5,
            from: 6,
            to: 4,
        },
        Operation {
            count: 14,
            from: 9,
            to: 2,
        },
        Operation {
            count: 1,
            from: 8,
            to: 9,
        },
        Operation {
            count: 1,
            from: 7,
            to: 8,
        },
        Operation {
            count: 1,
            from: 9,
            to: 6,
        },
        Operation {
            count: 2,
            from: 9,
            to: 5,
        },
        Operation {
            count: 1,
            from: 1,
            to: 2,
        },
        Operation {
            count: 7,
            from: 6,
            to: 9,
        },
        Operation {
            count: 1,
            from: 3,
            to: 4,
        },
        Operation {
            count: 8,
            from: 5,
            to: 2,
        },
        Operation {
            count: 1,
            from: 6,
            to: 7,
        },
        Operation {
            count: 1,
            from: 7,
            to: 4,
        },
        Operation {
            count: 1,
            from: 8,
            to: 4,
        },
        Operation {
            count: 1,
            from: 3,
            to: 9,
        },
        Operation {
            count: 7,
            from: 9,
            to: 5,
        },
        Operation {
            count: 1,
            from: 9,
            to: 1,
        },
        Operation {
            count: 6,
            from: 5,
            to: 1,
        },
        Operation {
            count: 8,
            from: 2,
            to: 4,
        },
        Operation {
            count: 1,
            from: 5,
            to: 6,
        },
        Operation {
            count: 1,
            from: 6,
            to: 7,
        },
        Operation {
            count: 1,
            from: 7,
            to: 9,
        },
        Operation {
            count: 7,
            from: 2,
            to: 9,
        },
        Operation {
            count: 1,
            from: 9,
            to: 4,
        },
        Operation {
            count: 3,
            from: 9,
            to: 1,
        },
        Operation {
            count: 1,
            from: 9,
            to: 6,
        },
        Operation {
            count: 11,
            from: 2,
            to: 8,
        },
        Operation {
            count: 9,
            from: 1,
            to: 8,
        },
        Operation {
            count: 1,
            from: 6,
            to: 4,
        },
        Operation {
            count: 1,
            from: 1,
            to: 9,
        },
        Operation {
            count: 12,
            from: 4,
            to: 2,
        },
        Operation {
            count: 4,
            from: 9,
            to: 3,
        },
        Operation {
            count: 3,
            from: 4,
            to: 6,
        },
        Operation {
            count: 9,
            from: 8,
            to: 6,
        },
        Operation {
            count: 12,
            from: 4,
            to: 9,
        },
        Operation {
            count: 8,
            from: 6,
            to: 3,
        },
        Operation {
            count: 8,
            from: 2,
            to: 7,
        },
        Operation {
            count: 11,
            from: 3,
            to: 4,
        },
        Operation {
            count: 2,
            from: 2,
            to: 7,
        },
        Operation {
            count: 2,
            from: 6,
            to: 1,
        },
        Operation {
            count: 1,
            from: 2,
            to: 3,
        },
        Operation {
            count: 2,
            from: 6,
            to: 2,
        },
        Operation {
            count: 3,
            from: 2,
            to: 6,
        },
        Operation {
            count: 2,
            from: 1,
            to: 6,
        },
        Operation {
            count: 1,
            from: 6,
            to: 1,
        },
        Operation {
            count: 1,
            from: 6,
            to: 4,
        },
        Operation {
            count: 2,
            from: 6,
            to: 3,
        },
        Operation {
            count: 1,
            from: 6,
            to: 5,
        },
        Operation {
            count: 4,
            from: 3,
            to: 8,
        },
        Operation {
            count: 12,
            from: 4,
            to: 5,
        },
        Operation {
            count: 5,
            from: 9,
            to: 7,
        },
        Operation {
            count: 3,
            from: 8,
            to: 7,
        },
        Operation {
            count: 1,
            from: 9,
            to: 1,
        },
        Operation {
            count: 3,
            from: 8,
            to: 2,
        },
        Operation {
            count: 13,
            from: 5,
            to: 6,
        },
        Operation {
            count: 1,
            from: 2,
            to: 9,
        },
        Operation {
            count: 13,
            from: 6,
            to: 7,
        },
        Operation {
            count: 7,
            from: 9,
            to: 6,
        },
        Operation {
            count: 2,
            from: 4,
            to: 6,
        },
        Operation {
            count: 1,
            from: 8,
            to: 6,
        },
        Operation {
            count: 1,
            from: 1,
            to: 6,
        },
        Operation {
            count: 1,
            from: 2,
            to: 9,
        },
        Operation {
            count: 1,
            from: 2,
            to: 3,
        },
        Operation {
            count: 12,
            from: 7,
            to: 9,
        },
        Operation {
            count: 7,
            from: 8,
            to: 4,
        },
        Operation {
            count: 1,
            from: 1,
            to: 3,
        },
        Operation {
            count: 2,
            from: 7,
            to: 9,
        },
        Operation {
            count: 15,
            from: 7,
            to: 4,
        },
        Operation {
            count: 8,
            from: 6,
            to: 3,
        },
        Operation {
            count: 1,
            from: 8,
            to: 9,
        },
        Operation {
            count: 1,
            from: 7,
            to: 2,
        },
        Operation {
            count: 10,
            from: 3,
            to: 5,
        },
        Operation {
            count: 6,
            from: 5,
            to: 9,
        },
        Operation {
            count: 1,
            from: 2,
            to: 8,
        },
        Operation {
            count: 1,
            from: 5,
            to: 8,
        },
        Operation {
            count: 2,
            from: 8,
            to: 9,
        },
        Operation {
            count: 10,
            from: 4,
            to: 9,
        },
        Operation {
            count: 20,
            from: 9,
            to: 6,
        },
        Operation {
            count: 1,
            from: 7,
            to: 6,
        },
        Operation {
            count: 4,
            from: 9,
            to: 3,
        },
        Operation {
            count: 1,
            from: 5,
            to: 9,
        },
        Operation {
            count: 4,
            from: 4,
            to: 9,
        },
        Operation {
            count: 8,
            from: 9,
            to: 7,
        },
        Operation {
            count: 2,
            from: 5,
            to: 1,
        },
        Operation {
            count: 7,
            from: 4,
            to: 3,
        },
        Operation {
            count: 8,
            from: 3,
            to: 2,
        },
        Operation {
            count: 6,
            from: 9,
            to: 8,
        },
        Operation {
            count: 1,
            from: 3,
            to: 7,
        },
        Operation {
            count: 1,
            from: 3,
            to: 1,
        },
        Operation {
            count: 7,
            from: 7,
            to: 8,
        },
        Operation {
            count: 13,
            from: 8,
            to: 3,
        },
        Operation {
            count: 2,
            from: 2,
            to: 8,
        },
        Operation {
            count: 1,
            from: 8,
            to: 2,
        },
        Operation {
            count: 1,
            from: 4,
            to: 1,
        },
        Operation {
            count: 1,
            from: 1,
            to: 8,
        },
        Operation {
            count: 2,
            from: 8,
            to: 2,
        },
        Operation {
            count: 24,
            from: 6,
            to: 2,
        },
        Operation {
            count: 2,
            from: 7,
            to: 8,
        },
        Operation {
            count: 5,
            from: 3,
            to: 4,
        },
        Operation {
            count: 25,
            from: 2,
            to: 6,
        },
        Operation {
            count: 5,
            from: 4,
            to: 9,
        },
        Operation {
            count: 2,
            from: 8,
            to: 7,
        },
        Operation {
            count: 2,
            from: 7,
            to: 3,
        },
        Operation {
            count: 4,
            from: 6,
            to: 2,
        },
        Operation {
            count: 2,
            from: 6,
            to: 4,
        },
        Operation {
            count: 9,
            from: 2,
            to: 3,
        },
        Operation {
            count: 11,
            from: 3,
            to: 7,
        },
        Operation {
            count: 10,
            from: 7,
            to: 8,
        },
        Operation {
            count: 1,
            from: 7,
            to: 9,
        },
        Operation {
            count: 3,
            from: 2,
            to: 4,
        },
        Operation {
            count: 8,
            from: 8,
            to: 2,
        },
        Operation {
            count: 1,
            from: 2,
            to: 6,
        },
        Operation {
            count: 2,
            from: 4,
            to: 1,
        },
        Operation {
            count: 1,
            from: 8,
            to: 2,
        },
        Operation {
            count: 1,
            from: 6,
            to: 9,
        },
        Operation {
            count: 1,
            from: 8,
            to: 3,
        },
        Operation {
            count: 6,
            from: 9,
            to: 7,
        },
        Operation {
            count: 2,
            from: 9,
            to: 1,
        },
        Operation {
            count: 9,
            from: 6,
            to: 8,
        },
        Operation {
            count: 7,
            from: 2,
            to: 3,
        },
        Operation {
            count: 7,
            from: 8,
            to: 2,
        },
        Operation {
            count: 10,
            from: 6,
            to: 8,
        },
        Operation {
            count: 7,
            from: 1,
            to: 2,
        },
        Operation {
            count: 9,
            from: 3,
            to: 2,
        },
        Operation {
            count: 5,
            from: 3,
            to: 8,
        },
        Operation {
            count: 4,
            from: 7,
            to: 2,
        },
        Operation {
            count: 2,
            from: 3,
            to: 2,
        },
        Operation {
            count: 12,
            from: 2,
            to: 3,
        },
        Operation {
            count: 6,
            from: 4,
            to: 2,
        },
        Operation {
            count: 1,
            from: 7,
            to: 6,
        },
        Operation {
            count: 5,
            from: 3,
            to: 5,
        },
        Operation {
            count: 16,
            from: 8,
            to: 4,
        },
        Operation {
            count: 12,
            from: 2,
            to: 7,
        },
        Operation {
            count: 5,
            from: 5,
            to: 7,
        },
        Operation {
            count: 1,
            from: 8,
            to: 3,
        },
        Operation {
            count: 1,
            from: 6,
            to: 4,
        },
        Operation {
            count: 17,
            from: 7,
            to: 4,
        },
        Operation {
            count: 1,
            from: 7,
            to: 1,
        },
        Operation {
            count: 1,
            from: 1,
            to: 9,
        },
        Operation {
            count: 1,
            from: 9,
            to: 5,
        },
        Operation {
            count: 11,
            from: 4,
            to: 9,
        },
        Operation {
            count: 10,
            from: 2,
            to: 3,
        },
        Operation {
            count: 1,
            from: 5,
            to: 4,
        },
        Operation {
            count: 1,
            from: 9,
            to: 2,
        },
        Operation {
            count: 2,
            from: 2,
            to: 1,
        },
        Operation {
            count: 1,
            from: 2,
            to: 3,
        },
        Operation {
            count: 23,
            from: 4,
            to: 5,
        },
        Operation {
            count: 7,
            from: 9,
            to: 7,
        },
        Operation {
            count: 3,
            from: 9,
            to: 1,
        },
        Operation {
            count: 20,
            from: 5,
            to: 6,
        },
        Operation {
            count: 3,
            from: 5,
            to: 8,
        },
        Operation {
            count: 1,
            from: 4,
            to: 1,
        },
        Operation {
            count: 2,
            from: 8,
            to: 3,
        },
        Operation {
            count: 4,
            from: 6,
            to: 4,
        },
        Operation {
            count: 7,
            from: 7,
            to: 2,
        },
        Operation {
            count: 1,
            from: 8,
            to: 4,
        },
        Operation {
            count: 19,
            from: 3,
            to: 9,
        },
        Operation {
            count: 5,
            from: 1,
            to: 7,
        },
        Operation {
            count: 7,
            from: 2,
            to: 6,
        },
        Operation {
            count: 3,
            from: 7,
            to: 5,
        },
        Operation {
            count: 2,
            from: 3,
            to: 4,
        },
        Operation {
            count: 1,
            from: 5,
            to: 4,
        },
        Operation {
            count: 1,
            from: 1,
            to: 4,
        },
        Operation {
            count: 1,
            from: 7,
            to: 6,
        },
        Operation {
            count: 13,
            from: 6,
            to: 7,
        },
        Operation {
            count: 6,
            from: 9,
            to: 3,
        },
        Operation {
            count: 1,
            from: 3,
            to: 5,
        },
        Operation {
            count: 2,
            from: 3,
            to: 4,
        },
        Operation {
            count: 2,
            from: 6,
            to: 2,
        },
        Operation {
            count: 3,
            from: 4,
            to: 3,
        },
        Operation {
            count: 8,
            from: 9,
            to: 1,
        },
        Operation {
            count: 2,
            from: 2,
            to: 1,
        },
        Operation {
            count: 8,
            from: 6,
            to: 7,
        },
        Operation {
            count: 2,
            from: 9,
            to: 4,
        },
        Operation {
            count: 20,
            from: 7,
            to: 1,
        },
        Operation {
            count: 2,
            from: 7,
            to: 5,
        },
        Operation {
            count: 2,
            from: 5,
            to: 1,
        },
        Operation {
            count: 8,
            from: 1,
            to: 8,
        },
        Operation {
            count: 8,
            from: 8,
            to: 6,
        },
        Operation {
            count: 1,
            from: 6,
            to: 9,
        },
        Operation {
            count: 8,
            from: 6,
            to: 1,
        },
        Operation {
            count: 1,
            from: 5,
            to: 3,
        },
        Operation {
            count: 7,
            from: 3,
            to: 2,
        },
        Operation {
            count: 1,
            from: 5,
            to: 2,
        },
        Operation {
            count: 2,
            from: 9,
            to: 7,
        },
        Operation {
            count: 1,
            from: 5,
            to: 8,
        },
        Operation {
            count: 18,
            from: 1,
            to: 4,
        },
        Operation {
            count: 1,
            from: 8,
            to: 9,
        },
        Operation {
            count: 3,
            from: 2,
            to: 3,
        },
        Operation {
            count: 2,
            from: 7,
            to: 4,
        },
        Operation {
            count: 5,
            from: 2,
            to: 4,
        },
        Operation {
            count: 3,
            from: 3,
            to: 8,
        },
        Operation {
            count: 8,
            from: 1,
            to: 7,
        },
        Operation {
            count: 2,
            from: 9,
            to: 2,
        },
        Operation {
            count: 32,
            from: 4,
            to: 5,
        },
        Operation {
            count: 1,
            from: 9,
            to: 7,
        },
        Operation {
            count: 1,
            from: 2,
            to: 1,
        },
        Operation {
            count: 6,
            from: 1,
            to: 6,
        },
        Operation {
            count: 1,
            from: 2,
            to: 4,
        },
        Operation {
            count: 3,
            from: 8,
            to: 1,
        },
        Operation {
            count: 3,
            from: 6,
            to: 5,
        },
        Operation {
            count: 1,
            from: 3,
            to: 6,
        },
        Operation {
            count: 2,
            from: 1,
            to: 9,
        },
        Operation {
            count: 4,
            from: 4,
            to: 7,
        },
        Operation {
            count: 31,
            from: 5,
            to: 4,
        },
        Operation {
            count: 4,
            from: 5,
            to: 6,
        },
        Operation {
            count: 1,
            from: 6,
            to: 1,
        },
        Operation {
            count: 7,
            from: 6,
            to: 5,
        },
        Operation {
            count: 1,
            from: 9,
            to: 4,
        },
        Operation {
            count: 19,
            from: 4,
            to: 2,
        },
        Operation {
            count: 1,
            from: 5,
            to: 9,
        },
        Operation {
            count: 5,
            from: 5,
            to: 6,
        },
        Operation {
            count: 3,
            from: 4,
            to: 2,
        },
        Operation {
            count: 2,
            from: 7,
            to: 1,
        },
        Operation {
            count: 4,
            from: 7,
            to: 8,
        },
        Operation {
            count: 3,
            from: 8,
            to: 6,
        },
        Operation {
            count: 2,
            from: 6,
            to: 7,
        },
        Operation {
            count: 6,
            from: 7,
            to: 8,
        },
        Operation {
            count: 3,
            from: 1,
            to: 5,
        },
        Operation {
            count: 4,
            from: 5,
            to: 9,
        },
        Operation {
            count: 15,
            from: 2,
            to: 1,
        },
        Operation {
            count: 4,
            from: 6,
            to: 4,
        },
        Operation {
            count: 2,
            from: 6,
            to: 3,
        },
        Operation {
            count: 1,
            from: 3,
            to: 7,
        },
        Operation {
            count: 4,
            from: 1,
            to: 2,
        },
        Operation {
            count: 1,
            from: 3,
            to: 4,
        },
        Operation {
            count: 2,
            from: 7,
            to: 4,
        },
        Operation {
            count: 5,
            from: 9,
            to: 3,
        },
        Operation {
            count: 2,
            from: 7,
            to: 3,
        },
        Operation {
            count: 16,
            from: 4,
            to: 8,
        },
        Operation {
            count: 8,
            from: 8,
            to: 5,
        },
        Operation {
            count: 2,
            from: 1,
            to: 5,
        },
        Operation {
            count: 1,
            from: 9,
            to: 6,
        },
        Operation {
            count: 1,
            from: 6,
            to: 5,
        },
        Operation {
            count: 7,
            from: 5,
            to: 9,
        },
        Operation {
            count: 3,
            from: 1,
            to: 8,
        },
        Operation {
            count: 1,
            from: 8,
            to: 4,
        },
        Operation {
            count: 8,
            from: 2,
            to: 7,
        },
        Operation {
            count: 3,
            from: 1,
            to: 3,
        },
        Operation {
            count: 1,
            from: 3,
            to: 9,
        },
        Operation {
            count: 2,
            from: 4,
            to: 2,
        },
        Operation {
            count: 7,
            from: 8,
            to: 5,
        },
        Operation {
            count: 7,
            from: 9,
            to: 1,
        },
        Operation {
            count: 6,
            from: 3,
            to: 5,
        },
        Operation {
            count: 6,
            from: 7,
            to: 4,
        },
        Operation {
            count: 3,
            from: 4,
            to: 1,
        },
        Operation {
            count: 3,
            from: 2,
            to: 5,
        },
        Operation {
            count: 1,
            from: 7,
            to: 8,
        },
        Operation {
            count: 1,
            from: 7,
            to: 5,
        },
        Operation {
            count: 1,
            from: 9,
            to: 8,
        },
        Operation {
            count: 2,
            from: 2,
            to: 4,
        },
        Operation {
            count: 15,
            from: 1,
            to: 6,
        },
        Operation {
            count: 8,
            from: 5,
            to: 9,
        },
        Operation {
            count: 3,
            from: 3,
            to: 4,
        },
        Operation {
            count: 4,
            from: 4,
            to: 3,
        },
        Operation {
            count: 1,
            from: 9,
            to: 7,
        },
        Operation {
            count: 6,
            from: 9,
            to: 4,
        },
        Operation {
            count: 1,
            from: 9,
            to: 2,
        },
        Operation {
            count: 6,
            from: 4,
            to: 9,
        },
        Operation {
            count: 2,
            from: 4,
            to: 6,
        },
        Operation {
            count: 5,
            from: 6,
            to: 9,
        },
        Operation {
            count: 1,
            from: 3,
            to: 1,
        },
        Operation {
            count: 8,
            from: 6,
            to: 8,
        },
        Operation {
            count: 12,
            from: 5,
            to: 3,
        },
        Operation {
            count: 1,
            from: 5,
            to: 3,
        },
        Operation {
            count: 1,
            from: 3,
            to: 8,
        },
        Operation {
            count: 4,
            from: 6,
            to: 1,
        },
        Operation {
            count: 11,
            from: 3,
            to: 8,
        },
        Operation {
            count: 1,
            from: 2,
            to: 1,
        },
        Operation {
            count: 23,
            from: 8,
            to: 2,
        },
        Operation {
            count: 3,
            from: 1,
            to: 2,
        },
        Operation {
            count: 1,
            from: 1,
            to: 9,
        },
        Operation {
            count: 2,
            from: 2,
            to: 3,
        },
        Operation {
            count: 6,
            from: 3,
            to: 6,
        },
        Operation {
            count: 1,
            from: 7,
            to: 6,
        },
        Operation {
            count: 1,
            from: 4,
            to: 7,
        },
        Operation {
            count: 1,
            from: 4,
            to: 3,
        },
        Operation {
            count: 1,
            from: 7,
            to: 3,
        },
        Operation {
            count: 4,
            from: 8,
            to: 4,
        },
        Operation {
            count: 2,
            from: 1,
            to: 8,
        },
        Operation {
            count: 3,
            from: 8,
            to: 1,
        },
        Operation {
            count: 4,
            from: 6,
            to: 2,
        },
        Operation {
            count: 7,
            from: 9,
            to: 1,
        },
        Operation {
            count: 1,
            from: 9,
            to: 6,
        },
        Operation {
            count: 2,
            from: 2,
            to: 3,
        },
        Operation {
            count: 3,
            from: 9,
            to: 4,
        },
        Operation {
            count: 1,
            from: 9,
            to: 3,
        },
        Operation {
            count: 10,
            from: 2,
            to: 8,
        },
        Operation {
            count: 16,
            from: 2,
            to: 5,
        },
        Operation {
            count: 2,
            from: 3,
            to: 6,
        },
        Operation {
            count: 6,
            from: 1,
            to: 8,
        },
        Operation {
            count: 1,
            from: 1,
            to: 5,
        },
        Operation {
            count: 8,
            from: 8,
            to: 5,
        },
        Operation {
            count: 11,
            from: 5,
            to: 9,
        },
        Operation {
            count: 2,
            from: 1,
            to: 8,
        },
        Operation {
            count: 1,
            from: 1,
            to: 8,
        },
        Operation {
            count: 4,
            from: 4,
            to: 6,
        },
        Operation {
            count: 3,
            from: 3,
            to: 9,
        },
        Operation {
            count: 14,
            from: 9,
            to: 3,
        },
        Operation {
            count: 15,
            from: 8,
            to: 5,
        },
        Operation {
            count: 9,
            from: 5,
            to: 4,
        },
        Operation {
            count: 7,
            from: 6,
            to: 1,
        },
        Operation {
            count: 1,
            from: 6,
            to: 3,
        },
        Operation {
            count: 4,
            from: 4,
            to: 7,
        },
        Operation {
            count: 2,
            from: 6,
            to: 2,
        },
        Operation {
            count: 4,
            from: 7,
            to: 4,
        },
        Operation {
            count: 4,
            from: 1,
            to: 4,
        },
        Operation {
            count: 10,
            from: 4,
            to: 3,
        },
        Operation {
            count: 14,
            from: 3,
            to: 6,
        },
        Operation {
            count: 5,
            from: 4,
            to: 1,
        },
        Operation {
            count: 6,
            from: 5,
            to: 7,
        },
        Operation {
            count: 1,
            from: 2,
            to: 6,
        },
        Operation {
            count: 3,
            from: 7,
            to: 2,
        },
        Operation {
            count: 2,
            from: 2,
            to: 3,
        },
        Operation {
            count: 3,
            from: 7,
            to: 8,
        },
        Operation {
            count: 2,
            from: 8,
            to: 2,
        },
        Operation {
            count: 2,
            from: 2,
            to: 7,
        },
        Operation {
            count: 6,
            from: 6,
            to: 2,
        },
        Operation {
            count: 1,
            from: 8,
            to: 7,
        },
        Operation {
            count: 8,
            from: 2,
            to: 7,
        },
        Operation {
            count: 1,
            from: 4,
            to: 1,
        },
        Operation {
            count: 5,
            from: 5,
            to: 3,
        },
        Operation {
            count: 3,
            from: 3,
            to: 2,
        },
        Operation {
            count: 5,
            from: 1,
            to: 3,
        },
        Operation {
            count: 7,
            from: 5,
            to: 8,
        },
        Operation {
            count: 6,
            from: 6,
            to: 3,
        },
        Operation {
            count: 1,
            from: 5,
            to: 9,
        },
        Operation {
            count: 10,
            from: 7,
            to: 9,
        },
        Operation {
            count: 26,
            from: 3,
            to: 4,
        },
        Operation {
            count: 1,
            from: 5,
            to: 1,
        },
        Operation {
            count: 6,
            from: 8,
            to: 2,
        },
        Operation {
            count: 9,
            from: 2,
            to: 9,
        },
        Operation {
            count: 1,
            from: 7,
            to: 5,
        },
        Operation {
            count: 1,
            from: 8,
            to: 5,
        },
        Operation {
            count: 2,
            from: 6,
            to: 2,
        },
        Operation {
            count: 20,
            from: 9,
            to: 6,
        },
        Operation {
            count: 1,
            from: 1,
            to: 6,
        },
        Operation {
            count: 1,
            from: 4,
            to: 2,
        },
        Operation {
            count: 1,
            from: 5,
            to: 8,
        },
        Operation {
            count: 1,
            from: 5,
            to: 7,
        },
        Operation {
            count: 3,
            from: 1,
            to: 3,
        },
        Operation {
            count: 1,
            from: 3,
            to: 6,
        },
        Operation {
            count: 12,
            from: 4,
            to: 8,
        },
        Operation {
            count: 11,
            from: 4,
            to: 5,
        },
        Operation {
            count: 1,
            from: 7,
            to: 5,
        },
        Operation {
            count: 1,
            from: 2,
            to: 8,
        },
        Operation {
            count: 1,
            from: 1,
            to: 8,
        },
        Operation {
            count: 2,
            from: 2,
            to: 5,
        },
        Operation {
            count: 8,
            from: 6,
            to: 2,
        },
        Operation {
            count: 5,
            from: 6,
            to: 4,
        },
        Operation {
            count: 2,
            from: 5,
            to: 3,
        },
        Operation {
            count: 12,
            from: 8,
            to: 4,
        },
        Operation {
            count: 5,
            from: 2,
            to: 6,
        },
        Operation {
            count: 3,
            from: 8,
            to: 1,
        },
        Operation {
            count: 11,
            from: 6,
            to: 8,
        },
        Operation {
            count: 10,
            from: 4,
            to: 6,
        },
        Operation {
            count: 5,
            from: 4,
            to: 6,
        },
        Operation {
            count: 12,
            from: 6,
            to: 5,
        },
        Operation {
            count: 22,
            from: 5,
            to: 6,
        },
        Operation {
            count: 3,
            from: 6,
            to: 5,
        },
        Operation {
            count: 3,
            from: 8,
            to: 5,
        },
        Operation {
            count: 1,
            from: 3,
            to: 8,
        },
        Operation {
            count: 4,
            from: 8,
            to: 1,
        },
        Operation {
            count: 6,
            from: 1,
            to: 7,
        },
        Operation {
            count: 5,
            from: 6,
            to: 9,
        },
    ];

    // Make it zero indexed, because for some cursed reason it isn't.
    for op in operations.iter_mut() {
      op.from -= 1;
      op.to -= 1;
    }
    return ([
        vec!['N', 'B', 'D', 'T', 'V', 'G', 'Z', 'J'],
        vec!['S', 'R', 'M', 'D', 'W', 'P', 'F'],
        vec!['V', 'C', 'R', 'S', 'Z'],
        vec!['R', 'T', 'J', 'Z', 'P', 'H', 'G'],
        vec!['T', 'C', 'J', 'N', 'D', 'Z', 'Q', 'F'],
        vec!['N', 'V', 'P', 'W', 'G', 'S', 'F', 'M'],
        vec!['G', 'C', 'V', 'B', 'P', 'Q'],
        vec!['Z', 'B', 'P', 'N'],
        vec!['W', 'P', 'J'],
    ], operations);
}
