use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nom_gcode::{DocComment, GCodeLine, Mnemonic, doc_comment, parse_gcode, parse_command, parse_args, ArgOrComment, parse_kv_arg};

fn criterion_benchmark(c: &mut Criterion) {
    let doc = ";Filament used: 0.943758m";
    let mut group = c.benchmark_group("doc_comment");
    group.bench_function("parse_gcode", |b| b.iter(|| {
        let gcode_line = parse_gcode(black_box(doc))
            .unwrap()
            .1
            .unwrap();

        if let GCodeLine::DocComment(DocComment::FilamentUsed { meters}) = gcode_line {
            assert_eq!(meters, 0.943758);
        } else {
            panic!("Expected a doc comment");
        }
    }));
    group.bench_function("doc_comment_only", |b| b.iter(|| {
        let gcode_line = doc_comment(black_box(doc))
            .unwrap()
            .1;

        if let DocComment::FilamentUsed { meters} = gcode_line {
            assert_eq!(meters, 0.943758);
        } else {
            panic!("Expected a doc comment");
        }
    }));
    group.finish();

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
            panic!("Expected a G1");
        }
    }));

    c.bench_function("parse_command g1", |b| b.iter(|| {
        let (_, gcode) = parse_command(black_box(g1))
            .unwrap();

        assert_eq!(gcode.line_number, None);
        assert_eq!(gcode.mnemonic, Mnemonic::General);
        assert_eq!(gcode.major, 1);
        assert_eq!(gcode.minor, 0);

        // let mut args = gcode.arguments();
        // assert_eq!(args.next(), g1_args.get(0));
        // assert_eq!(args.next(), g1_args.get(1));
        // assert_eq!(args.next(), g1_args.get(2));
        // assert_eq!(args.next(), None);
    }));
    c.bench_function("parse_command parse_args g1", |b| b.iter(|| {
        let (input, gcode) = parse_command(black_box(g1))
            .unwrap();

        assert_eq!(gcode.line_number, None);
        assert_eq!(gcode.mnemonic, Mnemonic::General);
        assert_eq!(gcode.major, 1);
        assert_eq!(gcode.minor, 0);

        let (_, args) = parse_args(false, input).unwrap();

        let mut args = args.unwrap()
            .into_iter()
            .filter_map(|arg| {
                if let ArgOrComment::KeyValue(key_value) = arg {
                    Some(key_value)
                } else {
                    None
                }
            });

        assert_eq!(args.next().as_ref(), g1_args.get(0));
        assert_eq!(args.next().as_ref(), g1_args.get(1));
        assert_eq!(args.next().as_ref(), g1_args.get(2));
        assert_eq!(args.next(), None);
    }));
    c.bench_function("parse_command parse_kv_arg loop g1", |b| b.iter(|| {
        let (mut input, gcode) = parse_command(black_box(g1))
            .unwrap();

        assert_eq!(gcode.line_number, None);
        assert_eq!(gcode.mnemonic, Mnemonic::General);
        assert_eq!(gcode.major, 1);
        assert_eq!(gcode.minor, 0);


        let mut arg_count = 0;
        loop {
            let arg = match parse_kv_arg(input) {
                Ok((rem, ArgOrComment::KeyValue(arg))) => {
                    input = rem;
                    Some(arg)
                }
                Ok(_) => continue,
                Err(_) => break,
            };

            assert_eq!(arg.as_ref(), g1_args.get(arg_count));
            arg_count += 1;
        }

        assert_eq!(arg_count, 3)
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
