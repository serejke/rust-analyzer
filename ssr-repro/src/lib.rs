pub fn foo(x: u32) -> u32 {
    x
}

pub fn bar(x: u32) -> u32 {
    x + 1
}

pub struct S;

impl S {
    pub fn step(&self) {}
    pub fn advance(&self) {}
}

pub fn f(s: &S) -> u32 {
    s.step();
    S::step(s);
    foo(1)
}
