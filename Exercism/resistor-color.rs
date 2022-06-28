#[derive(Debug, PartialEq)]
2
pub enum ResistorColor {
3
    Black,
4
    Blue,
5
    Brown,
6
    Green,
7
    Grey,
8
    Orange,
9
    Red,
10
    Violet,
11
    White,
12
    Yellow,
13
}
14
15
pub fn color_to_value(_color: ResistorColor) -> usize {
16
17
    match _color {
18
        ResistorColor::Black => 0,
19
        ResistorColor::Blue => 1,
20
        ResistorColor::Brown => 2,
21
        ResistorColor::Green => 3,
22
        ResistorColor::Grey => 4,
23
        ResistorColor::Orange => 5,
24
        ResistorColor::Red => 6,
25
        ResistorColor::Violet => 7,
26
        ResistorColor::White => 8,
27
        ResistorColor::Yellow => 9
28
    }
29
}
30
31
pub fn value_to_color_string(value: usize) -> String {
32
    match value {
33
        0 => "Black".to_string(),
34
        1 => "Blue".to_string(),
35
        2 => "Brown".to_string(),
36
        3 => "Green".to_string(),
37
        4 => "Grey".to_string(),
38
        5 => "Orange".to_string(),
39
        6 => "Red".to_string(),
40
        7 => "Violet".to_string(),
41
        8 => "White".to_string(),
42
        9 => "Yellow".to_string(),
43
        _ => "value out of range".to_string()
44
    }
45
}
46
47
pub fn colors() -> Vec<ResistorColor> {
48
    vec![
49
        ResistorColor::Black,
50
        ResistorColor::Brown,
51
        ResistorColor::Red,
52
        ResistorColor::Orange,
53
        ResistorColor::Yellow,
54
        ResistorColor::Green,
55
        ResistorColor::Blue,
56
        ResistorColor::Violet,
57
        ResistorColor::Grey,
58
        ResistorColor::White,
59
    ]
60
