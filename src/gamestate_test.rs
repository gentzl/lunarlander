#[cfg(test)]
mod tests {
    use macroquad::math::Vec2;

    use crate::{
        gamestate::{self, IMAGE_CORRECTION_Y},
        lunarmodule,
        map::SurfaceCoordinate,
    };

    fn get_surface_coordinates() -> Vec<crate::map::SurfaceCoordinate> {
        vec![
            SurfaceCoordinate {
                x: 100.0,
                y: 50.0,
                is_landing_zone_left: true,
                is_landing_zone_right: false,
            },
            SurfaceCoordinate {
                x: 130.0,
                y: 100.0,
                is_landing_zone_left: false,
                is_landing_zone_right: true,
            },
            SurfaceCoordinate {
                x: 160.0,
                y: 100.0,
                is_landing_zone_left: false,
                is_landing_zone_right: false,
            },
        ]
    }

    #[test]
    fn test_calculate_landed() {
        let surface_coordinates = get_surface_coordinates();
        let lunar_module = lunarmodule::LunarModule {
            rotation: 3.0,
            position: Vec2 {
                x: 101.0,
                y: 50.0 - IMAGE_CORRECTION_Y,
            },
            trust: 1.0,
            fuel: 100.0,
            trust_active: true,
        };
        let result = gamestate::calculate(lunar_module, &surface_coordinates);

        println!("result: {:?}", &result);

        assert_eq!(gamestate::GameState::Landed, result);
    }

    #[test]
    fn test_calculate_not_landed_to_high() {
        let surface_coordinates = get_surface_coordinates();
        let lunar_module = lunarmodule::LunarModule {
            rotation: 3.0,
            position: Vec2 { x: 101.0, y: 10.0 },
            trust: 1.0,
            fuel: 100.0,
            trust_active: true,
        };
        let result = gamestate::calculate(lunar_module, &surface_coordinates);

        println!("result: {:?}", &result);

        assert_eq!(gamestate::GameState::NotLanded, result);
    }

    #[test]
    fn test_calculate_not_landed_rotation_out_of_range() {
        let surface_coordinates = get_surface_coordinates();
        let lunar_module = lunarmodule::LunarModule {
            rotation: 15.0,
            position: Vec2 { x: 161.0, y: 80.0 },
            trust: 1.0,
            fuel: 100.0,
            trust_active: true,
        };
        let result = gamestate::calculate(lunar_module, &surface_coordinates);

        println!("result: {:?}", &result);

        assert_eq!(gamestate::GameState::NotLanded, result);
    }
}
