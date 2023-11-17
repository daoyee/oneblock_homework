enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// 定义一个 TrafficLightTrait trait
trait TrafficLightTrait {
    fn duration(&self) -> u8;
}

// 为 TrafficLight 枚举实现 TrafficLightTrait trait
impl TrafficLightTrait for TrafficLight {
    fn duration(&self) -> u8 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_traffic_light() {
        let red_light = TrafficLight::Red;
        let yellow_light = TrafficLight::Yellow;
        let green_light = TrafficLight::Green;

        assert_eq!(30, red_light.duration());
        assert_eq!(5, yellow_light.duration());
        assert_eq!(45, green_light.duration());
    }
}