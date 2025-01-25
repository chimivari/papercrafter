
#[derive(Debug)]
pub struct SymetricMatrix {
    m: [f64; 10],
}

impl SymetricMatrix {
    pub fn zeros() -> Self {
        Self {m: [0.; 10]}
    }
}