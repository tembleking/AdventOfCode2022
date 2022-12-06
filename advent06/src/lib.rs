#![allow(unused)]

use std::collections::VecDeque;

pub fn marker_position(mut input: impl Iterator<Item = char>) -> Option<usize> {
    let mut found_chars = VecDeque::with_capacity(4);

    for (step, c) in input.enumerate() {
        if found_chars.len() < 4 {
            found_chars.push_back(c);
            if found_chars.len() == 4 && all_chars_are_different(&found_chars) {
                return Some(step + 1);
            }
            continue;
        }

        found_chars.pop_front();
        found_chars.push_back(c);
        if all_chars_are_different(&found_chars) {
            return Some(step + 1);
        }
    }

    None
}

fn all_chars_are_different(chars: &VecDeque<char>) -> bool {
    for i in 0..chars.len() {
        for j in 0..chars.len() {
            if i != j && chars[i] == chars[j] {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::IndexMut;

    #[test]
    fn it_reports_that_no_marker_is_detected_because_no_enough_chars_have_been_received() {
        let input = "mjq".chars();

        let marker = marker_position(input);

        assert_eq!(marker, None);
    }

    #[test]
    fn it_reports_that_no_marker_is_detected_because_no_4_different_chars_were_detected() {
        let input = "mjqmjqmjqmjqmjqmjq".chars();

        let marker = marker_position(input);

        assert_eq!(marker, None);
    }

    #[test]
    fn it_reports_that_the_marker_is_found_at_position_7() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars();

        let marker = marker_position(input);

        assert_eq!(marker, Some(7));
    }

    #[test]
    fn it_reports_that_marker_is_found_at_pos_4() {
        let input = "mjqx".chars();

        let marker = marker_position(input);

        assert_eq!(marker, Some(4));
    }

    #[test]
    fn it_reports_marker_found_with_data_tests() {
        let data_tests = [
            "bvwbjplbgvbhsrlpgdmjqwftvncz".chars(),
            "nppdvjthqldpwncqszvftbrmjlhg".chars(),
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars(),
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars(),
        ];

        let results = [5_usize, 6, 10, 11];

        let markers = data_tests
            .into_iter()
            .filter_map(marker_position)
            .collect::<Vec<_>>();

        assert_eq!(markers, results);
    }
}
