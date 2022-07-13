mod analyze;

use self::analyze::Json4Analyze;

// fn parse(json: String) {
//     ()
// }

fn analyze(json: String) -> Json4Analyze {
    let mut analyze = Json4Analyze::new(json);
    analyze.analyze();
    return analyze;
}

#[test]
fn test_parse() {
    let json = "{
        \"a\" : {
            \"b\":1.2,
            \"b2\": \"a  aa\",
        },
        \"c\" : [
            2.2 , 2.3, 2.4, 2.5
        ],
        \"d\": false,
        \"e\":null
    }"
    .to_string();
    println!("{:?}", analyze(json))
}
