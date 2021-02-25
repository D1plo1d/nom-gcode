use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nom_gcode::{DocComment, GCodeLine, Mnemonic, parse_gcode};

fn criterion_benchmark(c: &mut Criterion) {
    let doc_comment = ";Filament used: 0.943758m";
    c.bench_function("parse_doc_comment", |b| b.iter(|| {
        let gcode_line = parse_gcode(black_box(doc_comment))
            .unwrap()
            .1
            .unwrap();

        if let GCodeLine::DocComment(DocComment::FilamentUsed { meters}) = gcode_line {
            assert_eq!(meters, 0.943758);
        } else {
            panic!("Expected a doc comment");
        }
    }));

    let g1 = "G1 X132.273 Y137.397 E3.64358";
    let g1_args = vec![
        ('X', Some(132.273)),
        ('Y', Some(137.397)),
        ('E', Some(3.64358)),
    ];
    c.bench_function("parse_g1", |b| b.iter(|| {
        let gcode_line = parse_gcode(black_box(g1))
            .unwrap()
            .1
            .unwrap();

        if let GCodeLine::GCode(gcode) = gcode_line {
            assert_eq!(gcode.line_number, None);
            assert_eq!(gcode.mnemonic, Mnemonic::General);
            assert_eq!(gcode.major, 1);
            assert_eq!(gcode.minor, 0);

            let mut args = gcode.arguments();
            assert_eq!(args.next(), g1_args.get(0));
            assert_eq!(args.next(), g1_args.get(1));
            assert_eq!(args.next(), g1_args.get(2));
            assert_eq!(args.next(), None);
        } else {
            panic!("Expected a doc comment");
        }
    }));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);