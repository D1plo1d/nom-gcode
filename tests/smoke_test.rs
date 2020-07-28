use nom_gcode::{
    // GCode,
    // Mnemonic,
    parse_gcode,
};

pub fn exec_smoke_test(src: &str) {
    src.lines().enumerate().for_each(|(i, line)| {
        let (remainder, _) = parse_gcode(&line)
            .expect(&format!("Failed to parse line #{}: {:?}\n\n", i + 1, line));

        assert!(
            remainder.len() == 0,
            "Failed to parse entire line. Line #{}, input: {} output: {}",
            i,
            line,
            remainder
        )
    });
}

macro_rules! smoke_test {
    ($name:ident, $filename:expr) => {
        #[test]
        // #[cfg(feature = "std")]
        fn $name() {
            let src = include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/data/",
                $filename
            ));
            exec_smoke_test(&src);
        }
    };
}

smoke_test!(program_1, "program_1.gcode");
smoke_test!(program_2, "program_2.gcode");
// smoke_test!(program_3, "program_3.gcode");
smoke_test!(program_4, "program_4.gcode");
smoke_test!(pi_octcat, "PI_octcat.gcode");
smoke_test!(pi_rustlogo, "PI_rustlogo.gcode");
// smoke_test!(insulpro_piping, "Insulpro.Piping.-.115mm.OD.-.40mm.WT.txt");

// #[test]
// #[ignore]
// fn expected_program_2_output() {
//     // N10 T2 M3 S447 F80
//     // N20 G0 X112 Y-2
//     // ;N30 Z-5
//     // N40 G41
//     // N50 G1 X95 Y8 M8
//     // ;N60 X32
//     // ;N70 X5 Y15
//     // ;N80 Y52
//     // N90 G2 X15 Y62 I10 J0
//     // N100 G1 X83
//     // N110 G3 X95 Y50 I12 J0
//     // N120 G1 Y-12
//     // N130 G40
//     // N140 G0 Z100 M9
//     // ;N150 X150 Y150
//     // N160 M30

//     let src = include_str!("data/program_2.gcode");

//     let got: Vec<_> =
//         gcode::full_parse_with_callbacks(src, PanicOnError).collect();

//     // total lines
//     assert_eq!(got.len(), 20);
//     // check lines without any comments
//     assert_eq!(got.iter().filter(|l| l.comments().is_empty()).count(), 11);

//     let gcodes: Vec<_> = got.iter().flat_map(|l| l.gcodes()).cloned().collect();
//     let expected = vec![
//         GCode::new(Mnemonic::ToolChange, 2.0, Span::PLACEHOLDER),
//         GCode::new(Mnemonic::Miscellaneous, 3.0, Span::PLACEHOLDER)
//             .with_argument(Word::new('S', 447.0, Span::PLACEHOLDER))
//             .with_argument(Word::new('F', 80.0, Span::PLACEHOLDER)),
//     ];
//     pretty_assertions::assert_eq!(gcodes, expected);
// }

