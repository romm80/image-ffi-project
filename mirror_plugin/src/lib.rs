use crate::params::MirrorParams;

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
    let mirror_params: MirrorParams = serde_json::from_str(params_str).unwrap_or_default();

    let mut top_y = 0u32;
    let mut bottom_y = height - 1;

    while top_y <= bottom_y {
        let mut left_x = 0u32;
        let mut right_x = width - 1;

        while left_x <= right_x {
            if mirror_params.horizontal {
                let i = (top_y * width + left_x) as usize * 4;
                let j = (top_y * width + right_x) as usize * 4;
                swap_point(data, i, j);

                if top_y != bottom_y {
                    let i = (bottom_y * width + left_x) as usize * 4;
                    let j = (bottom_y * width + right_x) as usize * 4;
                    swap_point(data, i, j);
                }
            }

            if mirror_params.vertical {
                let i = (top_y * width + left_x) as usize * 4;
                let j = (bottom_y * width + left_x) as usize * 4;
                swap_point(data, i, j);

                if left_x != right_x {
                    let i = (top_y * width + right_x) as usize * 4;
                    let j = (bottom_y * width + right_x) as usize * 4;
                    swap_point(data, i, j);
                }
            }

            if left_x == right_x {
                break;
            }
            left_x += 1;
            right_x -= 1;
        }

        if top_y == bottom_y {
            break;
        }
        top_y += 1;
        bottom_y -= 1;
    }
}

fn swap_point(data: &mut [u8], i: usize, j: usize) {
    data.swap(i, j); // R
    data.swap(i + 1, j + 1); // G
    data.swap(i + 2, j + 2); // B
    data.swap(i + 3, j + 3); // A
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    fn data() -> Vec<u8> {
        vec![
            255, 0, 0, 255, // (0,0) красный
            0, 255, 0, 255, // (1,0) зелёный
            0, 0, 255, 255, // (0,1) синий
            255, 255, 0, 255, // (1,1) жёлтый
        ]
    }

    #[test]
    fn test_horizontal_flip() {
        let mut data = data();
        let exp = vec![
            0, 255, 0, 255, // (0,0) зелёный
            255, 0, 0, 255, // (1,0) красный
            255, 255, 0, 255, // (0,1) жёлтый
            0, 0, 255, 255, // (1,1) синий
        ];

        let params = CString::new(r#"{"horizontal":true,"vertical":false}"#).unwrap();
        process_image(2, 2, data.as_mut_ptr(), params.as_ptr());
        assert_eq!(data, exp);
    }

    #[test]
    fn test_vertical_flip() {
        let mut data = data();
        let exp = vec![
            0, 0, 255, 255, // (0,0) синий
            255, 255, 0, 255, // (1,0) жёлтый
            255, 0, 0, 255, // (0,1) красный
            0, 255, 0, 255, // (1,1) зелёный
        ];

        let params = CString::new(r#"{"horizontal":false,"vertical":true}"#).unwrap();
        process_image(2, 2, data.as_mut_ptr(), params.as_ptr());
        assert_eq!(data, exp);
    }

    #[test]
    fn test_all_flip() {
        let mut data = data();
        let exp = vec![
            255, 255, 0, 255, // (0,0) жёлтый
            0, 0, 255, 255, // (1,0) синий
            0, 255, 0, 255, // (0,1) зелёный
            255, 0, 0, 255, // (1,1) красный
        ];

        let params = CString::new(r#"{"horizontal":true,"vertical":true}"#).unwrap();
        process_image(2, 2, data.as_mut_ptr(), params.as_ptr());
        assert_eq!(data, exp);
    }

    #[test]
    fn test_no_flip() {
        let mut data = data();
        let exp = vec![
            255, 0, 0, 255, // (0,0) красный
            0, 255, 0, 255, // (1,0) зелёный
            0, 0, 255, 255, // (0,1) синий
            255, 255, 0, 255, // (1,1) жёлтый
        ];

        let params = CString::new(r#"{"horizontal":false,"vertical":false}"#).unwrap();
        process_image(2, 2, data.as_mut_ptr(), params.as_ptr());
        assert_eq!(data, exp);
    }
}
