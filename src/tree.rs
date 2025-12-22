//! Simple, lossless syntax tree for TOML formatting.
//!
//! This module provides a custom tree structure that replaces Rowan,
//! optimized specifically for TOML formatting needs.

use crate::syntax::SyntaxKind;
use std::ops::Range;

pub type TextRange = Range<usize>;
pub type TextSize = usize;

// Helper function for creating TextRange (for compatibility with rowan)
pub fn text_range(start: TextSize, end: TextSize) -> TextRange {
    start..end
}

/// A complete syntax tree with source text
#[derive(Debug, Clone)]
pub struct SyntaxTree {
    pub root: Node,
    pub source: String,
}

/// A syntax tree node (e.g., ENTRY, TABLE_HEADER, etc.)
#[derive(Debug, Clone)]
pub struct Node {
    pub kind: SyntaxKind,
    pub span: TextRange,
    pub children: Vec<Element>,
}

/// Either a node or a token
#[derive(Debug, Clone)]
pub enum Element {
    Node(Node),
    Token(Token),
}

/// A leaf token (e.g., IDENT, STRING, COMMA, etc.)
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: SyntaxKind,
    pub span: TextRange,
}

impl Node {
    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }

    pub fn children(&self) -> &[Element] {
        &self.children
    }

    /// Iterator over children (for compatibility)
    pub fn children_with_tokens(&self) -> impl Iterator<Item = &Element> {
        self.children.iter()
    }

    pub fn text<'a>(&self, source: &'a str) -> &'a str {
        &source[self.span.clone()]
    }

    pub fn first_child(&self) -> Option<&Element> {
        self.children.first()
    }

    /// Iterator over all descendants (depth-first)
    pub fn descendants(&self) -> impl Iterator<Item = &Element> {
        DescendantsIter::new(self)
    }

    /// Iterator that includes both nodes and tokens
    pub fn descendants_with_tokens(&self) -> impl Iterator<Item = &Element> {
        self.descendants()
    }

    pub fn to_string(&self, source: &str) -> String {
        self.text(source).to_string()
    }
}

impl Token {
    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }

    pub fn text<'a>(&self, source: &'a str) -> &'a str {
        &source[self.span.clone()]
    }

    pub fn to_string(&self, source: &str) -> String {
        self.text(source).to_string()
    }
}

impl Element {
    pub fn kind(&self) -> SyntaxKind {
        match self {
            Element::Node(n) => n.kind(),
            Element::Token(t) => t.kind(),
        }
    }

    pub fn as_node(&self) -> Option<&Node> {
        match self {
            Element::Node(n) => Some(n),
            Element::Token(_) => None,
        }
    }

    pub fn as_token(&self) -> Option<&Token> {
        match self {
            Element::Node(_) => None,
            Element::Token(t) => Some(t),
        }
    }

    pub fn text<'a>(&self, source: &'a str) -> &'a str {
        match self {
            Element::Node(n) => n.text(source),
            Element::Token(t) => t.text(source),
        }
    }

    pub fn to_string(&self, source: &str) -> String {
        self.text(source).to_string()
    }

    pub fn text_range(&self) -> TextRange {
        match self {
            Element::Node(n) => n.span.clone(),
            Element::Token(t) => t.span.clone(),
        }
    }

    pub fn span(&self) -> &TextRange {
        match self {
            Element::Node(n) => &n.span,
            Element::Token(t) => &t.span,
        }
    }
}

impl From<Node> for Element {
    fn from(node: Node) -> Self {
        Element::Node(node)
    }
}

impl From<Token> for Element {
    fn from(token: Token) -> Self {
        Element::Token(token)
    }
}

/// Iterator for descendants (depth-first traversal)
struct DescendantsIter<'a> {
    stack: Vec<&'a Element>,
}

impl<'a> DescendantsIter<'a> {
    fn new(node: &'a Node) -> Self {
        Self { stack: node.children.iter().rev().collect() }
    }
}

impl<'a> Iterator for DescendantsIter<'a> {
    type Item = &'a Element;

    fn next(&mut self) -> Option<Self::Item> {
        let elem = self.stack.pop()?;
        if let Element::Node(node) = elem {
            self.stack.extend(node.children.iter().rev());
        }
        Some(elem)
    }
}

/// Builder for constructing a syntax tree during parsing
pub struct TreeBuilder {
    source: String,
    stack: Vec<NodeBuilder>,
    current_pos: usize,
}

struct NodeBuilder {
    kind: SyntaxKind,
    start: usize,
    children: Vec<Element>,
}

impl TreeBuilder {
    pub fn new(source: &str) -> Self {
        Self { source: source.to_string(), stack: Vec::new(), current_pos: 0 }
    }

    pub fn start_node(&mut self, kind: SyntaxKind) {
        self.stack.push(NodeBuilder { kind, start: self.current_pos, children: Vec::new() });
    }

    pub fn token(&mut self, kind: SyntaxKind, text: &str) {
        let span = self.current_pos..self.current_pos + text.len();
        let token = Token { kind, span };

        if let Some(parent) = self.stack.last_mut() {
            parent.children.push(Element::Token(token));
        }

        self.current_pos += text.len();
    }

    pub fn finish_node(&mut self) {
        let builder = self.stack.pop().expect("finish_node called without start_node");
        let node = Node {
            kind: builder.kind,
            span: builder.start..self.current_pos,
            children: builder.children,
        };

        if let Some(parent) = self.stack.last_mut() {
            parent.children.push(Element::Node(node));
        } else {
            // This is the root - push it back as a completed root
            self.stack.push(NodeBuilder {
                kind: builder.kind,
                start: builder.start,
                children: vec![Element::Node(node)],
            });
        }
    }

    pub fn finish(mut self) -> SyntaxTree {
        assert_eq!(self.stack.len(), 1, "TreeBuilder finished with unbalanced nodes");

        let root_builder = self.stack.pop().unwrap();
        let root = match root_builder.children.into_iter().next() {
            Some(Element::Node(n)) => n,
            _ => panic!("TreeBuilder finished without root node"),
        };

        SyntaxTree { root, source: self.source }
    }
}

impl SyntaxTree {
    /// Get the root node
    pub fn root(&self) -> &Node {
        &self.root
    }

    /// Get the source text
    pub fn source(&self) -> &str {
        &self.source
    }
}
