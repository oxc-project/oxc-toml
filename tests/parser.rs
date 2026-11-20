use oxc_toml::{SyntaxElement, SyntaxKind, parse};

#[test]
fn test_array_table_header_whitespace() {
    // Whitespace may surround the key, but not split `[[` / `]]`
    for input in ["[[ a ]]\n", "[[\ta.b\t]]\n"] {
        assert!(parse(input).errors.is_empty(), "input: {input:?}");
    }
    for input in ["[ [a]]\n", "[[a] ]\n"] {
        assert!(!parse(input).errors.is_empty(), "input: {input:?}");
    }
}

#[test]
fn test_trivia_after_scalar_value_is_its_parents() {
    // A `VALUE` holds the value only, the whitespace and comment after it belong to the parent
    for input in [
        "a = 1 # c\n",
        "a = [1 # c\n, 2 ]\n",
        "a = { b = 1 , c = 'x' }\n",
        "a = [\"\"\"x\ny\"\"\" # c\n]\n",
    ] {
        let parsed = parse(input);
        assert!(parsed.errors.is_empty(), "input: {input:?}");
        for element in parsed.root.descendants() {
            if let SyntaxElement::Node(node) = element
                && node.kind() == SyntaxKind::VALUE
            {
                assert!(
                    node.children()
                        .iter()
                        .all(|e| !matches!(e.kind(), SyntaxKind::WHITESPACE | SyntaxKind::COMMENT)),
                    "input: {input:?}, value: {:?}",
                    node.children()
                );
            }
        }
    }
}
