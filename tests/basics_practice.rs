#[test]
fn minutes() {
    use swarmladder_rust::basics_practice::integer_arithmetic::minutes;
    assert_eq!(minutes(59), 0);
    assert_eq!(minutes(120), 2);
}

#[test]
fn clock() {
    use swarmladder_rust::basics_practice::integer_arithmetic::clock;
    assert_eq!(clock(3661), (1, 1, 1));
    assert_eq!(clock(86400), (24, 0, 0));
}

#[test]
fn area() {
    use swarmladder_rust::basics_practice::integer_arithmetic::area;
    assert_eq!(area(0, 8), 0);
    assert_eq!(area(65535, 65535), 4294836225);
}

#[test]
fn last_digit() {
    use swarmladder_rust::basics_practice::integer_arithmetic::last_digit;
    assert_eq!(last_digit(-12), 2);
    assert_eq!(last_digit(i32::MIN), 8);
}

#[test]
fn ceil_minutes() {
    use swarmladder_rust::basics_practice::integer_arithmetic::ceil_minutes;
    assert_eq!(ceil_minutes(60), 1);
    assert_eq!(ceil_minutes(61), 2);
    assert_eq!(ceil_minutes(u32::MAX), 71582789);
}

#[test]
fn fare() {
    use swarmladder_rust::basics_practice::decisions_and_ranges::fare;
    assert_eq!(fare(5), 0);
    assert_eq!(fare(6), 2);
    assert_eq!(fare(18), 5);
    assert_eq!(fare(65), 3);
}

#[test]
fn leap() {
    use swarmladder_rust::basics_practice::decisions_and_ranges::leap;
    assert_eq!(leap(1900), false);
    assert_eq!(leap(2000), true);
    assert_eq!(leap(2024), true);
}

#[test]
fn band() {
    use swarmladder_rust::basics_practice::decisions_and_ranges::band;
    assert_eq!(band(80), Some('B'));
    assert_eq!(band(90), Some('A'));
    assert_eq!(band(101), None);
}

#[test]
fn inside() {
    use swarmladder_rust::basics_practice::decisions_and_ranges::inside;
    assert_eq!(inside(3, 5, 3), true);
    assert_eq!(inside(1, 2, 2), false);
}

#[test]
fn quadrant() {
    use swarmladder_rust::basics_practice::decisions_and_ranges::quadrant;
    assert_eq!(quadrant(-1, 4), Some(2));
    assert_eq!(quadrant(5, -2), Some(4));
    assert_eq!(quadrant(0, -1), None);
}

#[test]
fn gcd() {
    use swarmladder_rust::basics_practice::loops_and_termination::gcd;
    assert_eq!(gcd(18, 12), 6);
    assert_eq!(gcd(0, 7), 7);
    assert_eq!(gcd(0, 0), 0);
}

#[test]
fn digits() {
    use swarmladder_rust::basics_practice::loops_and_termination::digits;
    assert_eq!(digits(0), 1);
    assert_eq!(digits(100), 3);
}

#[test]
fn factorial() {
    use swarmladder_rust::basics_practice::loops_and_termination::factorial;
    assert_eq!(factorial(0), Some(1));
    assert_eq!(factorial(5), Some(120));
    assert_eq!(factorial(21), None);
}

#[test]
fn powers() {
    use swarmladder_rust::basics_practice::loops_and_termination::powers;
    assert_eq!(powers(0), vec![]);
    assert_eq!(powers(9), vec![1, 2, 4, 8]);
}

#[test]
fn digit_sum() {
    use swarmladder_rust::basics_practice::loops_and_termination::digit_sum;
    assert_eq!(digit_sum(0), 0);
    assert_eq!(digit_sum(709), 16);
}

#[test]
fn parcel() {
    use swarmladder_rust::basics_practice::tuples_and_records::parcel;
    use swarmladder_rust::basics_practice::Parcel;
    assert_eq!(
        parcel(3, 200),
        Parcel {
            pieces: 3,
            unit_grams: 200
        }
    );
    assert_eq!(
        parcel(0, 5),
        Parcel {
            pieces: 0,
            unit_grams: 5
        }
    );
}

#[test]
fn weight() {
    use swarmladder_rust::basics_practice::tuples_and_records::weight;
    use swarmladder_rust::basics_practice::Parcel;
    assert_eq!(
        weight(&Parcel {
            pieces: 4,
            unit_grams: 90
        }),
        360
    );
    assert_eq!(
        weight(&Parcel {
            pieces: 0,
            unit_grams: 9
        }),
        0
    );
}

#[test]
fn heavier() {
    use swarmladder_rust::basics_practice::tuples_and_records::heavier;
    use swarmladder_rust::basics_practice::Parcel;
    assert_eq!(
        heavier(
            &Parcel {
                pieces: 3,
                unit_grams: 5
            },
            &Parcel {
                pieces: 1,
                unit_grams: 14
            }
        ),
        true
    );
    assert_eq!(
        heavier(
            &Parcel {
                pieces: 0,
                unit_grams: 2
            },
            &Parcel {
                pieces: 0,
                unit_grams: 9
            }
        ),
        false
    );
}

#[test]
fn unpack() {
    use swarmladder_rust::basics_practice::tuples_and_records::unpack;
    use swarmladder_rust::basics_practice::Parcel;
    assert_eq!(
        unpack(Parcel {
            pieces: 2,
            unit_grams: 150
        }),
        (2, 150)
    );
    assert_eq!(
        unpack(Parcel {
            pieces: 0,
            unit_grams: 0
        }),
        (0, 0)
    );
}

#[test]
fn combine() {
    use swarmladder_rust::basics_practice::tuples_and_records::combine;
    use swarmladder_rust::basics_practice::Parcel;
    assert_eq!(
        combine(
            Parcel {
                pieces: 2,
                unit_grams: 4
            },
            Parcel {
                pieces: 3,
                unit_grams: 4
            }
        ),
        Some(Parcel {
            pieces: 5,
            unit_grams: 4
        })
    );
    assert_eq!(
        combine(
            Parcel {
                pieces: 2,
                unit_grams: 3
            },
            Parcel {
                pieces: 1,
                unit_grams: 4
            }
        ),
        None
    );
}

#[test]
fn badge() {
    use swarmladder_rust::basics_practice::strings_and_formatting::badge;
    assert_eq!(badge(" Ada ", 7), "Ada #0007");
    assert_eq!(badge("Q", 12345), "Q #12345");
}

#[test]
fn subject() {
    use swarmladder_rust::basics_practice::strings_and_formatting::subject;
    assert_eq!(subject(" A: b:c "), Some(("A", "b:c")));
    assert_eq!(subject("abc"), None);
}

#[test]
fn initials() {
    use swarmladder_rust::basics_practice::strings_and_formatting::initials;
    assert_eq!(initials("Ada  Łukasz"), "AŁ");
    assert_eq!(initials(" "), "");
}

#[test]
fn reverse_text() {
    use swarmladder_rust::basics_practice::strings_and_formatting::reverse_text;
    assert_eq!(reverse_text("aé🙂"), "🙂éa");
    assert_eq!(reverse_text(""), "");
}

#[test]
fn underline() {
    use swarmladder_rust::basics_practice::strings_and_formatting::underline;
    assert_eq!(underline("é!"), "--");
    assert_eq!(underline(""), "");
}

#[test]
fn ends() {
    use swarmladder_rust::basics_practice::borrowing_slices::ends;
    assert_eq!(ends(&[]), None);
    assert_eq!(ends(&[7]), Some((7, 7)));
}

#[test]
fn suffix() {
    use swarmladder_rust::basics_practice::borrowing_slices::suffix;
    assert_eq!(suffix(&[1, 2, 3], 2), &[2, 3]);
    assert_eq!(suffix(&[3], 5), &[3]);
}

#[test]
fn split_at_checked() {
    use swarmladder_rust::basics_practice::borrowing_slices::split_at_checked;
    assert_eq!(split_at_checked(&[1, 2], 1), Some((&[1][..], &[2][..])));
    assert_eq!(split_at_checked(&[], 1), None);
}

#[test]
fn swap_ends() {
    use swarmladder_rust::basics_practice::borrowing_slices::swap_ends;
    let mut a = [4, 2, 8];
    assert!(swap_ends(&mut a));
    assert_eq!(a, [8, 2, 4]);
    assert!(!swap_ends(&mut []));
}

#[test]
fn shift() {
    use swarmladder_rust::basics_practice::borrowing_slices::shift;
    let mut a = [1, 2, 3];
    shift(&mut a, 4);
    assert_eq!(a, [2, 3, 1]);
    shift(&mut [], 2);
}

#[test]
fn next() {
    use swarmladder_rust::basics_practice::enums_and_matching::next;
    use swarmladder_rust::basics_practice::Signal;
    assert_eq!(next(Signal::Amber), Signal::Red);
    assert_eq!(next(Signal::Red), Signal::Green);
}

#[test]
fn signal() {
    use swarmladder_rust::basics_practice::enums_and_matching::signal;
    use swarmladder_rust::basics_practice::Signal;
    assert_eq!(signal("amber"), Some(Signal::Amber));
    assert_eq!(signal("Red"), None);
}

#[test]
fn may_go() {
    use swarmladder_rust::basics_practice::enums_and_matching::may_go;
    use swarmladder_rust::basics_practice::Signal;
    assert_eq!(may_go(Signal::Amber), false);
    assert_eq!(may_go(Signal::Green), true);
}

#[test]
fn advance() {
    use swarmladder_rust::basics_practice::enums_and_matching::advance;
    use swarmladder_rust::basics_practice::Signal;
    assert_eq!(advance(Signal::Red, 0), Signal::Red);
    assert_eq!(advance(Signal::Red, 5), Signal::Amber);
}

#[test]
fn green_count() {
    use swarmladder_rust::basics_practice::enums_and_matching::green_count;
    use swarmladder_rust::basics_practice::Signal;
    assert_eq!(green_count(&[Signal::Red, Signal::Green, Signal::Amber]), 1);
    assert_eq!(green_count(&[]), 0);
}

#[test]
fn sorted() {
    use swarmladder_rust::basics_practice::small_algorithms::sorted;
    assert_eq!(sorted(&[2, 2, 3]), true);
    assert_eq!(sorted(&[3, 1]), false);
}

#[test]
fn streak() {
    use swarmladder_rust::basics_practice::small_algorithms::streak;
    assert_eq!(streak(&[1, 1, 2, 1], 1), 2);
    assert_eq!(streak(&[], 5), 0);
}

#[test]
fn changes() {
    use swarmladder_rust::basics_practice::small_algorithms::changes;
    assert_eq!(changes(&[1, 1, 2, 1]), 2);
    assert_eq!(changes(&[4]), 0);
}

#[test]
fn palindrome() {
    use swarmladder_rust::basics_practice::small_algorithms::palindrome;
    assert_eq!(palindrome("é🙂é"), true);
    assert_eq!(palindrome("Aa"), false);
}

#[test]
fn merge_pairs() {
    use swarmladder_rust::basics_practice::small_algorithms::merge_pairs;
    assert_eq!(merge_pairs(&[1, 2, 3], &[9]), vec![1, 9, 2, 3]);
    assert_eq!(merge_pairs(&[], &[4]), vec![4]);
}
