use cgmath::prelude::*;
#[cfg(test)]
mod tests {

    use cgmath::Basis3;
    use cgmath::Deg;
    use cgmath::Rotation3;
    use cgmath::Vector3;

    use super::*;
    #[test]
    fn test_move_lunar_module_up_increases_trust() {
        let v1 = Vector3::new(1.0, 1.0, 0.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);

        let result = v1 - v2;
        println!("result: {:?}", result);

        let rad_angle = v1.angle(v2);
        let angle_degree = Deg::from(rad_angle);

        let axis = Vector3::new(1.0, 1.0, 0.0).normalize();
        let ax: Basis3<_> = Rotation3::from_axis_angle(axis, Deg(30.0));
        //let vec =let Vector2::From(a);
        println!("ax: {:?}", ax);
        println!("v1: {:?}", v1);
        println!("v2: {:?}", v2);
        println!("angle (rad): {:?}", rad_angle);
        println!("angle (°): {:?}", angle_degree);
    }
}
