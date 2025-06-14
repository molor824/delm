use glam::{Vec3, Vec4};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(pub Vec4);

impl Color {
    pub fn from_hsva_vec(hsva: Vec4) -> Self {
        let hue_color = hue_to_rgb(hsva.x);
        let saturated_color = Vec3::ONE.lerp(hue_color, hsva.y.clamp(0.0, 1.0));
        Self((saturated_color * hsva.z.clamp(0.0, 1.0)).extend(hsva.w.clamp(0.0, 1.0)))
    }
    pub fn from_hsla_vec(hsla: Vec4) -> Self {
        let hue_color = hue_to_rgb(hsla.x);
        let saturated_color = Vec3::splat(0.5).lerp(hue_color, hsla.y.clamp(0.0, 1.0));

        Self(
            if hsla.z <= 0.5 {
                Vec3::ZERO.lerp(saturated_color, hsla.z.max(0.0) * 2.0)
            } else {
                saturated_color.lerp(Vec3::ONE, hsla.z.min(1.0) * 2.0 - 1.0)
            }
            .extend(hsla.w.clamp(0.0, 1.0)),
        )
    }
}

fn hue_to_rgb(mut hue: f32) -> Vec3 {
    hue = hue.rem_euclid(360.0);
    Vec3::new(
        match hue {
            300.0..360.0 | 0.0..60.0 => 1.0,
            60.0..120.0 => -hue / 60.0 + 2.0,
            240.0..300.0 => hue / 60.0 - 4.0,
            _ => 0.0,
        },
        match hue {
            60.0..180.0 => 1.0,
            0.0..60.0 => hue / 60.0,
            180.0..240.0 => -hue / 60.0 + 4.0,
            _ => 0.0,
        },
        match hue {
            180.0..300.0 => 1.0,
            120.0..180.0 => hue / 60.0 - 2.0,
            300.0..360.0 => -hue / 60.0 + 6.0,
            _ => 0.0,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::vec4;

    #[test]
    fn test() {
        assert_eq!(
            Color::from_hsla_vec(vec4(0.0, 1.0, 0.5, 1.0)),
            Color(vec4(1.0, 0.0, 0.0, 1.0))
        );
    }
}
