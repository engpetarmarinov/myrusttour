pub fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    THREE_HOURS_IN_SECONDS = 6; //error[E0070]: invalid left-hand side of assignment
    let x = 5;
    x = 6; //error[E0384]: cannot assign twice to immutable variable `x`
}
