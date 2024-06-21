#[cfg(test)]
mod tests {
    use macroquad::math::Vec2;

    use crate::{lunarmodule, movement};

    #[test]
    fn test_calculate_position_straight_y_up() {
        let result = movement::calculate_position(&lunarmodule::LunarModule {
            rotation: 0.0,
            position: Vec2 { x: 50.0, y: 50.0 },
            trust: 1.0,
            fuel: 100.0,
            trust_active: true,
        });

        println!("result: {:?}", result);

        assert_eq!(51.0, result.y);
        assert_eq!(50.0, result.x);
    }

    #[test]
    fn test_calculate_position_straight_x_right() {
        let result = movement::calculate_position(&lunarmodule::LunarModule {
            rotation: 90.0,
            position: Vec2 { x: 50.0, y: 50.0 },
            trust: 1.0,
            fuel: 100.0,
            trust_active: true,
        });

        println!("result: {:?}", result);

        assert_eq!(52.0, result.y);
        assert_eq!(51.0, result.x);
    }

    #[test]
    fn test_calculate_position_straight_x_left() {
        let result = movement::calculate_position(&lunarmodule::LunarModule {
            rotation: 270.0,
            position: Vec2 { x: 50.0, y: 50.0 },
            trust: 1.0,
            fuel: 100.0,
            trust_active: true,
        });

        println!("result: {:?}", result);

        assert_eq!(52.0, result.y);
        assert_eq!(49.0, result.x);
    }

    #[test]
    fn test_calculate_position_straight_y_down() {
        let result = movement::calculate_position(&lunarmodule::LunarModule {
            rotation: 180.0,
            position: Vec2 { x: 50.0, y: 50.0 },
            trust: 1.0,
            fuel: 100.0,
            trust_active: true,
        });

        println!("result: {:?}", result);

        assert_eq!(53.0, result.y);
        assert_eq!(50.0, result.x);
    }

    #[test]
    fn test_calculate_position_straight_45degrees() {
        let result = movement::calculate_position(&lunarmodule::LunarModule {
            rotation: 45.0,
            position: Vec2 { x: 50.0, y: 50.0 },
            trust: 1.0,
            fuel: 100.0,
            trust_active: true,
        });

        println!("result: {:?}", result);

        assert_eq!(51.292892, result.y);
        assert_eq!(50.7071075, result.x);
    }
}
