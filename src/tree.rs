//! Simple, lossless syntax tree for TOML formatting.
//!
//! This module provides a custom tree structure that replaces Rowan,
//! optimized specifically for TOML formatting needs.

use crate::syntax::SyntaxKind;
use std::ops::Range;

/// Byte offsets into the source. Limited to 4 GiB of source text.
pub type TextRange = Range<u32>;

#[inline]
pub const fn text_range(start: usize, end: usize) -> TextRange {
    start as u32..end as u32
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
    pub const fn kind(&self) -> SyntaxKind {
        self.kind
    }

    pub fn children(&self) -> &[Element] {
        &self.children
    }

    /// Iterator over children (for compatibility)
    pub fn children_with_tokens(&self) -> impl Iterator<Item = &Element> {
        self.children.iter()
    }

    #[inline]
    pub fn text<'a>(&self, source: &'a str) -> &'a str {
        &source[self.span.start as usize..self.span.end as usize]
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
    pub const fn kind(&self) -> SyntaxKind {
        self.kind
    }

    #[inline]
    pub fn text<'a>(&self, source: &'a str) -> &'a str {
        &source[self.span.start as usize..self.span.end as usize]
    }

    pub fn to_string(&self, source: &str) -> String {
        self.text(source).to_string()
    }
}

impl Element {
    pub const fn kind(&self) -> SyntaxKind {
        match self {
            Element::Node(n) => n.kind(),
            Element::Token(t) => t.kind(),
        }
    }

    pub const fn as_node(&self) -> Option<&Node> {
        match self {
            Element::Node(n) => Some(n),
            Element::Token(_) => None,
        }
    }

    pub const fn as_token(&self) -> Option<&Token> {
        match self {
            Element::Node(_) => None,
            Element::Token(t) => Some(t),
        }
    }

    #[inline]
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

    pub const fn span(&self) -> &TextRange {
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
    stack: Vec<NodeBuilder>,
    current_pos: usize,
}

struct NodeBuilder {
    kind: SyntaxKind,
    start: usize,
    children: Vec<Element>,
}

impl TreeBuilder {
    pub fn new() -> Self {
        // TOML nesting is shallow; 8 is enough to avoid reallocation in typical files.
        Self { stack: Vec::with_capacity(8), current_pos: 0 }
    }

    pub fn start_node(&mut self, kind: SyntaxKind) {
        // Pre-size children based on the node kind to avoid the typical 0->4 growth.
        let cap = match kind {
            SyntaxKind::KEY | SyntaxKind::VALUE => 1,
            SyntaxKind::ENTRY | SyntaxKind::TABLE_HEADER | SyntaxKind::TABLE_ARRAY_HEADER => 4,
            _ => 0,
        };
        let children = if cap == 0 { Vec::new() } else { Vec::with_capacity(cap) };
        self.stack.push(NodeBuilder { kind, start: self.current_pos, children });
    }

    pub fn token(&mut self, kind: SyntaxKind, text: &str) {
        let end = self.current_pos + text.len();
        let token = Token { kind, span: self.current_pos as u32..end as u32 };

        if let Some(parent) = self.stack.last_mut() {
            parent.children.push(Element::Token(token));
        }

        self.current_pos = end;
    }

    pub fn finish_node(&mut self) {
        let builder = self.stack.pop().expect("finish_node called without start_node");
        let node = Node {
            kind: builder.kind,
            span: builder.start as u32..self.current_pos as u32,
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

    /// Finalize the tree by returning the root node. The source is supplied separately
    /// so the builder does not need to own a copy during parsing.
    pub fn finish_root(mut self) -> Node {
        assert_eq!(self.stack.len(), 1, "TreeBuilder finished with unbalanced nodes");

        let root_builder = self.stack.pop().unwrap();
        match root_builder.children.into_iter().next() {
            Some(Element::Node(n)) => n,
            _ => panic!("TreeBuilder finished without root node"),
        }
    }
}

impl SyntaxTree {
    /// Get the root node
    pub const fn root(&self) -> &Node {
        &self.root
    }

    /// Get the source text
    pub fn source(&self) -> &str {
        &self.source
    }
}
