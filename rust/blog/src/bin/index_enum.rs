use std::ops::Index;

fn main() {
    let p = BasketBallTeam {
        point_guard: BasketBallPlayer {
            name: "",
            position: BasketBallPosition::PointGuard,
        },
        shooting_guard: BasketBallPlayer {
            name: "",
            position: BasketBallPosition::ShootingGuard,
        },
        center: BasketBallPlayer {
            name: "",
            position: BasketBallPosition::Center,
        },
        power_forward: BasketBallPlayer {
            name: "",
            position: BasketBallPosition::PowerForward,
        },
        small_forward: BasketBallPlayer {
            name: "",
            position: BasketBallPosition::SmallForward,
        },
    };

    let m = &[BasketBallPosition::PointGuard];
}

enum BasketBallPosition {
    PointGuard,
    ShootingGuard,
    Center,
    PowerForward,
    SmallForward,
}

struct BasketBallPlayer {
    name: &'static str,
    position: BasketBallPosition,
}

struct BasketBallTeam {
    point_guard: BasketBallPlayer,
    shooting_guard: BasketBallPlayer,
    center: BasketBallPlayer,
    power_forward: BasketBallPlayer,
    small_forward: BasketBallPlayer,
}

impl Index<BasketBallPosition> for BasketBallTeam {
    type Output = BasketBallPlayer;

    fn index(&self, position: BasketBallPosition) -> &Self::Output {
        match position {
            BasketBallPosition::PointGuard => &self.point_guard,
            BasketBallPosition::ShootingGuard => &self.shooting_guard,
            BasketBallPosition::Center => &self.center,
            BasketBallPosition::PowerForward => &self.power_forward,
            BasketBallPosition::SmallForward => &self.small_forward,
        }
    }
}
