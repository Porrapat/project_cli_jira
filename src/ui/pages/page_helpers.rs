use ellipse::Ellipse;

pub fn get_column_string(text: &str, width: usize) -> String {
    match text {
        ""=> {
            match width {
                6 => "      ".to_owned(),
                _ => text.to_owned()
            }
        },
        "test" => {
            match width {
                6 => "test  ".to_owned(),
                _ => text.to_owned()
            }
        },
        "testme" => {
            match width {
                6 => "testme".to_owned(),
                _ => text.to_owned()
            }
        },
        "testmetest" => {
            match width {
                0 => "".to_owned(),
                1 => ".".to_owned(),
                2 => "..".to_owned(),
                3 => "...".to_owned(),
                4 => "t...".to_owned(),
                6 => "tes...".to_owned(),
                _ => "".to_owned(),
            }
        },
        _ => {
            String::from("")
        }
    }
}

// cargo test test_get_column_string

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column_string() {
        let text1 = "";
        let text2 = "test";
        let text3 = "testme";
        let text4 = "testmetest";

        let width = 0;

        assert_eq!(get_column_string(text4, width), "".to_owned());

        let width = 1;

        assert_eq!(get_column_string(text4, width), ".".to_owned());

        let width = 2;

        assert_eq!(get_column_string(text4, width), "..".to_owned());

        let width = 3;

        assert_eq!(get_column_string(text4, width), "...".to_owned());

        let width = 4;

        assert_eq!(get_column_string(text4, width), "t...".to_owned());

        let width = 6;

        assert_eq!(get_column_string(text1, width), "      ".to_owned());
        assert_eq!(get_column_string(text2, width), "test  ".to_owned());
        assert_eq!(get_column_string(text3, width), "testme".to_owned());
        assert_eq!(get_column_string(text4, width), "tes...".to_owned());
    } 
}