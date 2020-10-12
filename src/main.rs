use semver::*;

fn main() {
    println!("Hello, world!");
    let sources: Vec<&'static str> = vec![
        "10000000.0.0-a+b",
        "a",
        "1-alpha+3",
        "x.y-beta.alpha+unicorn.rhino",
        "x.y-beta.alpha+unicorn.rhino+1234",
    ];

    for s in sources {
        let source = Source::from_str(s);

        let mut parser = Parser::new(source.clone());

        let version = parser.parse_all::<ast::Version>().unwrap();
        println!("{:?}", version);
        println!("major: {:?}", source.text(&version.major));
        println!("minor: {:?}", source.text(&version.minor));
        println!("patch: {:?}", source.text(&version.patch));
        println!("pre: {:?}", source.text(&version.pre));
        println!("build_meta: {:?}", source.text(&version.build_meta));
        println!();
    }

    println!("{}", std::mem::size_of::<ast::Version>());
}
