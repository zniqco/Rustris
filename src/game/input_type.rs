#[derive(Clone, Copy)]
pub enum InputType {
    MoveLeft = 0,
    MoveRight,
    SoftDrop,
    HardDrop,
    RotateCCW,
    RotateCW,
    Flip,
    Hold,
    _Max,
}
