use colorsys::Hsl;

// Parse a colour parameter in the form: `{key}:{val}` where `{val}` is a float
pub fn parse_colour_param(p: &str, key: &str) -> Option<f32> {
    let parts: Vec<&str> = p.split(":").collect();
    if parts.len() != 2 {
        return None;
    }

    let k = parts[0];
    let v = parts[1];
    if key != k {
        return None;
    }

    v.parse::<f32>().ok()
}

#[allow(clippy::assertions_on_constants)]
fn pct_to_hue(pct: f32) -> f32 {
    const LEFT_STOP: f32 = 0.0; // Red
    const RIGHT_STOP: f32 = 120.0; // Green
    assert!(RIGHT_STOP > LEFT_STOP);
    // Note: pct is inverted as we want to transition from green to red
    LEFT_STOP + ((1.0 - pct) * (RIGHT_STOP - LEFT_STOP))
}

const DEFAULT_SATURATION: f32 = 100.0;
const DEFAULT_LIGHTNESS: f32 = 50.0;

pub fn pct_value_hsl(norm: f32, s: Option<f32>, l: Option<f32>) -> Hsl {
    let hue = pct_to_hue(norm);
    let s = s.unwrap_or(DEFAULT_SATURATION);
    let l = l.unwrap_or(DEFAULT_LIGHTNESS);
    Hsl::from((hue, s, l))
}

#[cfg(test)]
mod tests {
    use super::*;
    use colorsys::Rgb;

    #[test]
    fn test_hsl_to_rgb_red() {
        let hsl = Hsl::from((0.0, 100.0, 50.0));
        let rgb: [u8; 3] = Rgb::from(&hsl).into();
        assert_eq!((255, 0, 0), rgb.into());
    }

    #[test]
    fn test_hsl_to_rgb_green() {
        let hsl = Hsl::from((120.0, 100.0, 50.0));
        let rgb: [u8; 3] = Rgb::from(&hsl).into();
        assert_eq!((0, 255, 0), rgb.into());
    }
}
