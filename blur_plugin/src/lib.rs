use crate::params::BlurParams;

mod params;

#[unsafe(no_mangle)]
extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const libc::c_char,
) {
    // SAFETY: вызывающая сторона гарантирует что указатель не null
    // память не освобождена до использования
    // длина массива = width * height * 4
    let data = unsafe { std::slice::from_raw_parts_mut(rgba_data, (width * height * 4) as usize) };

    // SAFETY: вызывающая сторона гарантирует что указатель не null
    // память не освобождена до использования
    // строка должна быть null-terminated (\0 в конце)
    let params_str = unsafe { std::ffi::CStr::from_ptr(params).to_str().unwrap_or("") };
    let blur_params: BlurParams = serde_json::from_str(params_str).unwrap_or_default();
    let iterations = blur_params.iterations;
    let radius = blur_params.radius;

    for _ in 0..iterations {
        let data_copy = data.to_vec();
        for x in 0..width {
            for y in 0..height {
                let mut sum_r = 0f64;
                let mut sum_g = 0f64;
                let mut sum_b = 0f64;
                let mut total_weight = 0f64;

                let mut i = (x as i64 - radius as i64).clamp(0, width as i64 - 1);
                let max_i = (x as i64 + radius as i64).clamp(0, width as i64 - 1);
                while i <= max_i {
                    let mut j = (y as i64 - radius as i64).clamp(0, height as i64 - 1);
                    let max_j = (y as i64 + radius as i64).clamp(0, height as i64 - 1);
                    while j <= max_j {
                        let dx = i.abs_diff(x as i64);
                        let dy = j.abs_diff(y as i64);
                        let distance = ((dx.pow(2) + dy.pow(2)) as f64).sqrt();
                        if distance <= radius as f64 {
                            let weight = 1.0 / (1.0 + distance);
                            total_weight += weight;
                            let idx = (j as u32 * width + i as u32) as usize * 4;

                            sum_r += data_copy[idx] as f64 * weight;
                            sum_g += data_copy[idx + 1] as f64 * weight;
                            sum_b += data_copy[idx + 2] as f64 * weight;
                        }
                        j += 1;
                    }
                    i += 1;
                }
                let idx = (y * width + x) as usize * 4;
                data[idx] = (sum_r / total_weight) as u8;
                data[idx + 1] = (sum_g / total_weight) as u8;
                data[idx + 2] = (sum_b / total_weight) as u8;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    fn uniform_image(color: [u8; 4], count: usize) -> Vec<u8> {
        color.iter().cloned().cycle().take(count * 4).collect()
    }

    #[test]
    fn test_uniform_blur() {
        let mut data = uniform_image([100, 150, 200, 255], 9); // 3x3

        // после blur ожидаем то же самое
        let exp = uniform_image([100, 150, 200, 255], 9); // 3x3

        let params = CString::new(r#"{"radius":20,"iterations":2}"#).unwrap();
        process_image(3, 3, data.as_mut_ptr(), params.as_ptr());

        // погрешность при обрезании float
        for (a, b) in data.iter().zip(exp.iter()) {
            assert!(a.abs_diff(*b) <= 1, "pixel diff too large: {a} vs {b}");
        }
    }

    #[test]
    fn test_nonuniform_blur_changes_pixels() {
        let white = [255u8, 255, 255, 255];
        let black = [0u8, 0, 0, 255];

        let mut data = vec![
            // строка 0
            white[0], white[1], white[2], white[3],
            white[0], white[1], white[2], white[3],
            white[0], white[1], white[2], white[3],
            // строка 1
            white[0], white[1], white[2], white[3],
            black[0], black[1], black[2], black[3],  // центр
            white[0], white[1], white[2], white[3],
            // строка 2
            white[0], white[1], white[2], white[3],
            white[0], white[1], white[2], white[3],
            white[0], white[1], white[2], white[3],
        ];

        let center_before = data[16]; // R центрального пикселя (индекс 1*3+1 = 4, *4 = 16)
        let params = CString::new(r#"{"radius":1,"iterations":1}"#).unwrap();
        process_image(3, 3, data.as_mut_ptr(), params.as_ptr());

        assert_ne!(data[16], center_before, "blur should change center pixel");
    }
}
