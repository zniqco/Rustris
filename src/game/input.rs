pub struct Input {
    previous: [bool; 8],
    current: [bool; 8],
    horizontal: i32,
    pub move_left: bool,
    pub move_right: bool,
    pub soft_drop: bool,
    pub hard_drop: bool,
    pub rotate_cw: bool,
    pub rotate_ccw: bool,
    pub flip: bool,
    pub hold: bool,
}

impl Input {
    pub fn new() -> Input {
        Self {
            previous: [false; 8],
            current: [false; 8],
            horizontal: 0,
            move_left: false,
            move_right: false,
            soft_drop: false,
            hard_drop: false,
            rotate_cw: false,
            rotate_ccw: false,
            flip: false,
            hold: false,
        }
    }

    fn holded(&self, index: usize) -> bool {
        self.current[index]
    }

    fn pressed(&self, index: usize) -> bool {
        !self.previous[index] && self.current[index]
    }

    fn released(&self, index: usize) -> bool {
        self.previous[index] && !self.current[index]
    }

    pub fn update(&mut self) {
        self.previous = self.current;
        self.current = [
            self.move_left,
            self.move_right,
            self.soft_drop,
            self.hard_drop,
            self.rotate_cw,
            self.rotate_ccw,
            self.flip,
            self.hold,
        ];

        let pressed = if self.pressed(0) { -1 } else { 0 } + if self.pressed(1) { 1 } else { 0 };

        if pressed != 0 {
            self.horizontal = pressed;
        }

        if self.released(0) && self.horizontal == -1 || self.released(1) && self.horizontal == 1 {
            self.horizontal = 0;
        }
    }

    pub fn horizontal(&mut self) -> i32 {
        self.horizontal
    }

    pub fn soft_drop(&mut self) -> bool {
        self.holded(2)
    }

    pub fn hard_drop(&mut self) -> bool {
        self.pressed(3)
    }
    
    pub fn cw(&mut self) -> bool {
        self.pressed(4)
    }
    
    pub fn ccw(&mut self) -> bool {
        self.pressed(5)
    }

    pub fn flip(&mut self) -> bool {
        self.pressed(6)
    }

    pub fn hold(&mut self) -> bool {
        self.pressed(7)
    }
}
