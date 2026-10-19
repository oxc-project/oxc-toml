//! This module is used to format TOML.
//!
//! The formatting can be done on documents that might
//! contain invalid syntax. In that case the invalid part is skipped.

use crate::{
    parser::Parse,
    syntax::{SyntaxElement, SyntaxKind::*, SyntaxNode, SyntaxToken},
    tree::{Element, TextRange},
    util::overlaps,
};
use std::cell::OnceCell;
use std::{cmp, collections::VecDeque, ops::Range, rc::Rc};

/// Simplified Keys struct for tracking table paths (used for indentation)
#[derive(Debug, Clone, PartialEq, Eq)]
struct Keys {
    keys: Vec<String>,
}

impl Keys {
    fn from_syntax(syntax: &SyntaxElement, source: &str) -> Self {
        let mut keys = Vec::new();
        if let Some(node) = syntax.as_node() {
            for child in node.children_with_tokens() {
                if child.kind() == IDENT {
                    keys.push(child.to_string(source));
                }
            }
        }
        Self { keys }
    }

    /// Check if current key contains (is nested under) another key
    fn contains(&self, other: &Keys) -> bool {
        if other.keys.len() > self.keys.len() {
            return false;
        }
        self.keys.iter().zip(&other.keys).all(|(a, b)| a == b)
    }
}

/// All the formatting options.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Options {
    /// Align entries vertically.
    ///
    /// Entries that have table headers, comments,
    /// or blank lines between them are not aligned.
    pub align_entries: bool,

    /// Align consecutive comments after entries and items vertically.
    ///
    /// This applies to comments that are after entries or array items.
    pub align_comments: bool,

    /// If `align_comments` is true, apply the alignment in cases where
    /// there's only one comment.
    pub align_single_comments: bool,

    /// Put trailing commas for multiline
    /// arrays.
    pub array_trailing_comma: bool,

    /// Automatically expand arrays to multiple lines once they
    /// exceed the configured `column_width`.
    pub array_auto_expand: bool,

    /// Expand values (e.g.) inside inline tables
    /// where possible.
    pub inline_table_expand: bool,

    pub inline_table_auto_expand: bool,

    /// Automatically collapse arrays if they
    /// fit in one line.
    ///
    /// The array won't be collapsed if it
    /// contains a comment.
    pub array_auto_collapse: bool,

    /// Omit whitespace padding inside single-line arrays.
    pub compact_arrays: bool,

    /// Omit whitespace padding inside inline tables.
    pub compact_inline_tables: bool,

    /// Omit whitespace around `=`.
    pub compact_entries: bool,

    /// Target maximum column width after which
    /// arrays are expanded into new lines.
    ///
    /// This is best-effort and might not be accurate.
    pub column_width: usize,

    /// Indent subtables if they come in order.
    pub indent_tables: bool,

    /// Indent entries under tables.
    pub indent_entries: bool,

    /// Indentation to use, should be tabs or spaces
    /// but technically could be anything.
    pub indent_string: String,

    /// Add trailing newline to the source.
    pub trailing_newline: bool,

    /// Alphabetically reorder keys that are not separated by blank lines.
    pub reorder_keys: bool,

    /// Alphabetically reorder array values that are not separated by blank lines.
    pub reorder_arrays: bool,

    /// Alphabetically reorder inline table values.
    pub reorder_inline_tables: bool,

    /// The maximum amount of consecutive blank lines allowed.
    pub allowed_blank_lines: usize,

    /// Use CRLF line endings
    pub crlf: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            align_entries: false,
            align_comments: false,
            align_single_comments: false,
            array_trailing_comma: true,
            array_auto_expand: true,
            array_auto_collapse: true,
            compact_arrays: true,
            compact_inline_tables: false,
            compact_entries: false,
            column_width: 100,
            indent_tables: false,
            indent_entries: false,
            inline_table_expand: true,
            inline_table_auto_expand: false,
            trailing_newline: true,
            allowed_blank_lines: 2,
            indent_string: "  ".into(),
            reorder_keys: false,
            reorder_arrays: false,
            reorder_inline_tables: false,
            crlf: false,
        }
    }
}

impl Options {
    const fn newline(&self) -> &'static str {
        if self.crlf { "\r\n" } else { "\n" }
    }

    fn newlines(&self, count: usize) -> impl Iterator<Item = &'static str> {
        std::iter::repeat_n(self.newline(), usize::min(count, self.allowed_blank_lines + 1))
    }

    const fn should_align_comments(&self, comment_count: usize) -> bool {
        (comment_count != 1 || self.align_single_comments) && self.align_comments
    }
}

#[derive(Debug, Clone)]
struct Context {
    indent_level: usize,
    force_multiline: bool,
    errors: Rc<[TextRange]>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            indent_level: Default::default(),
            force_multiline: Default::default(),
            errors: Rc::from([]),
        }
    }
}

impl Context {
    fn error_at(&self, range: TextRange) -> bool {
        for error_range in self.errors.iter() {
            if overlaps(range.clone(), error_range.clone()) {
                return true;
            }
        }

        false
    }

    fn indent<'o>(&self, opts: &'o Options) -> impl Iterator<Item = &'o str> {
        std::iter::repeat_n(opts.indent_string.as_ref(), self.indent_level)
    }
}

/// Formats a parsed TOML document, skipping ranges that contain syntax errors.
pub fn format(parse: &Parse, options: Options) -> String {
    let ctx = Context {
        errors: parse.errors.iter().map(|err| err.range.clone()).collect(),
        ..Context::default()
    };

    let mut formatted = format_root(&parse.root, parse.source, &options, &ctx);

    if formatted.ends_with("\r\n") {
        formatted.truncate(formatted.len() - 2);
    } else if formatted.ends_with('\n') {
        formatted.truncate(formatted.len() - 1);
    }

    if options.trailing_newline {
        formatted += options.newline();
    }

    formatted
}

struct FormattedEntry<'a> {
    syntax: &'a SyntaxNode,
    key: String,
    /// This field is used to cache the "cleaned" version of the key and should only
    /// be accessed through the `cleaned_key` helpers method.
    cleaned_key: OnceCell<Vec<String>>,
    value: String,
    comment: Option<String>,
}

impl FormattedEntry<'_> {
    fn cleaned_key(&self) -> &Vec<String> {
        self.cleaned_key.get_or_init(|| {
            self.key.replace(['\'', '"'], "").split('.').map(ToOwned::to_owned).collect()
        })
    }
}

impl PartialEq for FormattedEntry<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cleaned_key().eq(other.cleaned_key())
    }
}

impl Eq for FormattedEntry<'_> {}

impl PartialOrd for FormattedEntry<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FormattedEntry<'_> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.cleaned_key().cmp(other.cleaned_key())
    }
}

impl FormattedEntry<'_> {
    fn write_to(&self, formatted: &mut String, options: &Options) {
        *formatted += &self.key;
        if options.compact_entries {
            *formatted += "=";
        } else {
            *formatted += " = ";
        }
        *formatted += &self.value;
    }
}

fn format_root(node: &SyntaxNode, source: &str, options: &Options, context: &Context) -> String {
    assert!(node.kind() == ROOT);
    // Output size is roughly proportional to the input.
    let mut formatted = String::with_capacity(source.len());

    let mut entry_group: Vec<FormattedEntry> = Vec::new();

    // We defer printing the entries so that we can align them vertically.
    // Whenever an entry is added to the group, we skip its trailing newline,
    // otherwise the inserted new line would end up before the actual entries.
    let mut skip_newlines = 0;

    // We defer printing comments as well because we need to know
    // what comes after them for correct indentation.
    let mut comment_group: Vec<String> = Vec::new();

    let mut context = context.clone();

    // Table key for determining indents
    let mut table_key_indent_history: Vec<(Keys, usize)> = Vec::new();

    fn add_comments(
        comments: &mut Vec<String>,
        formatted: &mut String,
        context: &Context,
        options: &Options,
    ) -> bool {
        let were_comments = !comments.is_empty();

        for (idx, comment) in comments.drain(0..).enumerate() {
            if idx != 0 {
                *formatted += options.newline();
            }
            formatted.extend(context.indent(options));
            *formatted += &comment;
        }

        were_comments
    }

    let mut dangling_newline_count = 0;

    for c in node.children_with_tokens() {
        if context.error_at(c.text_range()) {
            formatted += c.text(source);
            continue;
        }

        match c {
            Element::Node(node) => match node.kind() {
                TABLE_ARRAY_HEADER | TABLE_HEADER => {
                    if add_entries(source, &mut entry_group, &mut formatted, options, &context) {
                        formatted += options.newline();
                        skip_newlines = 0;
                    }

                    // We treat everything as indented other than table headers from now on.
                    if options.indent_entries && context.indent_level == 0 {
                        context.indent_level = 1;
                    }

                    if let Some(key) = node.first_child().map(|e| Keys::from_syntax(e, source)) {
                        if options.indent_tables {
                            context.indent_level = table_indent_level(
                                &table_key_indent_history,
                                &key,
                                if options.indent_entries { 1 } else { 0 },
                            );
                        }
                        table_key_indent_history.push((key.clone(), context.indent_level));
                    }

                    let mut header_context = context.clone();

                    if options.indent_entries {
                        header_context.indent_level = header_context.indent_level.saturating_sub(1);
                    }

                    if add_comments(&mut comment_group, &mut formatted, &header_context, options) {
                        formatted += options.newline();
                        skip_newlines = 0;
                    }

                    if options.indent_tables {
                        formatted.extend(header_context.indent(options));
                    }

                    let comment =
                        format_table_header(node, source, options, &header_context, &mut formatted);
                    if let Some(c) = comment {
                        formatted += " ";
                        formatted += &c;
                    }
                }
                ENTRY => {
                    if add_comments(&mut comment_group, &mut formatted, &context, options) {
                        formatted += options.newline();
                        skip_newlines = 0;
                    }

                    entry_group.push(format_entry(node, source, options, &context));
                    skip_newlines += 1;
                }
                _ => unreachable!(),
            },
            Element::Token(token) => match token.kind() {
                NEWLINE => {
                    let mut newline_count = token.text(source).newline_count();

                    match dangling_newlines(token, source) {
                        Some(dnl) => {
                            dangling_newline_count += dnl;
                            continue;
                        }
                        None => {
                            newline_count += dangling_newline_count;
                            dangling_newline_count = 0;
                        }
                    }

                    if newline_count > 1 {
                        add_comments(&mut comment_group, &mut formatted, &context, options);
                        add_entries(source, &mut entry_group, &mut formatted, options, &context);
                        skip_newlines = 0;
                    }

                    formatted.extend(options.newlines(newline_count.saturating_sub(skip_newlines)));
                }
                COMMENT => {
                    if add_entries(source, &mut entry_group, &mut formatted, options, &context) {
                        formatted += options.newline();
                        skip_newlines = 0;
                    }
                    comment_group.push(token.text(source).to_string());
                    skip_newlines += 1;
                }
                WHITESPACE => {}
                _ => formatted += token.text(source),
            },
        }
    }

    add_comments(&mut comment_group, &mut formatted, &context, options);
    add_entries(source, &mut entry_group, &mut formatted, options, &context);

    formatted
}

/// Determine the indentation level using the indentation history.
///
/// The latest key that is a strict prefix is used and indented. If none is found, the default
/// indentation is used.
fn table_indent_level(
    history: &[(Keys, usize)],
    current_key: &Keys,
    default_indent: usize,
) -> usize {
    history
        .iter()
        .rev()
        .find_map(|(previous_key, indent)| {
            (current_key.contains(previous_key) && current_key != previous_key)
                .then_some(*indent + 1)
        })
        .unwrap_or(default_indent)
}

/// Add entries to the formatted string.
fn add_entries(
    source: &str,
    entry_group: &mut Vec<FormattedEntry>,
    formatted: &mut String,
    options: &Options,
    context: &Context,
) -> bool {
    let were_entries = !entry_group.is_empty();

    if options.reorder_keys {
        entry_group.sort();
    }

    let indent_chars_count = context.indent_level * options.indent_string.chars().count();

    // We check for too long lines, and try to expand them if possible.
    // We don't take vertical alignment into account for simplicity.
    if options.array_auto_expand || options.inline_table_auto_expand {
        for entry in entry_group.iter_mut() {
            let prefix = indent_chars_count + entry_prefix_chars(&entry.key, options);
            if exceeds_width(prefix, &entry.value, comment_chars(entry.comment.as_deref()), options)
            {
                let mut context = context.clone();
                context.force_multiline = true;

                // too long, reformat the value of the entry
                entry.value.clear();
                let comment = format_value(
                    entry
                        .syntax
                        .children()
                        .iter()
                        .find(|n| n.kind() == VALUE)
                        .and_then(|e| e.as_node())
                        .unwrap(),
                    source,
                    options,
                    &context,
                    &mut entry.value,
                );

                if let Some(c) = comment {
                    debug_assert!(entry.comment.is_none() || entry.comment.as_ref().unwrap() == &c);
                    entry.comment = Some(c);
                }
            }
        }
    }

    // Fast path: when neither entry alignment nor comment alignment is enabled,
    // we can skip building intermediate rows and write directly.
    if !options.align_entries && !options.align_comments {
        let separator = if options.compact_entries { "=" } else { " = " };
        // Pre-compute indent once per group instead of per entry.
        let indent_str: String = context.indent(options).collect();
        let newline = options.newline();
        for (i, e) in entry_group.drain(0..).enumerate() {
            if i != 0 {
                *formatted += newline;
            }
            *formatted += &indent_str;
            *formatted += &e.key;
            *formatted += separator;
            *formatted += &e.value;
            if let Some(c) = e.comment {
                *formatted += " ";
                *formatted += &c;
            }
        }
        return were_entries;
    }

    let mut comment_count = 0;
    // Transform the entries into generic rows that can be aligned.
    let rows = entry_group
        .drain(0..)
        .map(|e| {
            let mut row = Vec::with_capacity(5);

            row.push(context.indent(options).collect::<String>());
            row.push(e.key);
            row.push("=".to_string());
            row.push(e.value);
            if let Some(c) = e.comment {
                row.push(c);
                comment_count += 1;
            }

            row
        })
        .collect::<Vec<_>>();

    let align_comments = options.should_align_comments(comment_count);
    format_rows(
        formatted,
        if !options.align_entries && !align_comments {
            0..0
        } else if !options.align_entries && align_comments {
            3..usize::MAX
        } else if options.align_entries && !align_comments {
            0..3
        } else {
            0..usize::MAX
        },
        if options.compact_entries { 3..usize::MAX } else { 1..usize::MAX },
        &rows,
        options.newline(),
        " ",
    );

    were_entries
}

fn format_entry<'a>(
    node: &'a SyntaxNode,
    source: &str,
    options: &Options,
    context: &Context,
) -> FormattedEntry<'a> {
    let mut key = String::new();
    let mut value = String::new();
    let mut comment = None;

    for c in node.children_with_tokens() {
        match c {
            Element::Node(n) => match n.kind() {
                KEY => {
                    format_key(n, source, &mut key, options, context);
                }
                VALUE => {
                    let c = format_value(n, source, options, context, &mut value);
                    // In TOML 1.1, entries can have comments - only take first
                    if c.is_some() && comment.is_none() {
                        comment = c;
                    }
                }
                _ => unreachable!(),
            },
            Element::Token(t) => {
                if let COMMENT = t.kind() {
                    // In TOML 1.1, entries can have comments - only take first
                    if comment.is_none() {
                        comment = Some(t.text(source).into());
                    }
                }
            }
        }
    }

    FormattedEntry { syntax: node, key, cleaned_key: OnceCell::new(), value, comment }
}

fn format_key(
    node: &SyntaxNode,
    source: &str,
    formatted: &mut String,
    _options: &Options,
    _context: &Context,
) {
    // Idents and periods without whitespace
    for c in node.children_with_tokens() {
        match c {
            Element::Node(_) => {}
            Element::Token(t) => match t.kind() {
                WHITESPACE | NEWLINE => {}
                _ => {
                    *formatted += t.text(source);
                }
            },
        }
    }
}

/// Format a VALUE node by writing its value to `value` and returning its trailing comment, if any.
fn format_value(
    node: &SyntaxNode,
    source: &str,
    options: &Options,
    context: &Context,
    value: &mut String,
) -> Option<String> {
    let mut comment = None;
    for c in node.children_with_tokens() {
        match c {
            Element::Node(n) => match n.kind() {
                ARRAY => {
                    if let Some(c) = format_array(n, source, options, context, value) {
                        debug_assert!(comment.is_none());
                        comment = Some(c);
                    }
                }
                INLINE_TABLE => {
                    if let Some(c) = format_inline_table(n, source, options, context, value) {
                        debug_assert!(comment.is_none());
                        comment = Some(c);
                    }
                }
                _ => unreachable!(),
            },
            Element::Token(t) => match t.kind() {
                NEWLINE | WHITESPACE => {}
                COMMENT => {
                    debug_assert!(comment.is_none());
                    comment = Some(t.text(source).into());
                }
                _ => {
                    value.push_str(t.text(source));
                }
            },
        }
    }

    comment
}

fn format_inline_table(
    node: &SyntaxNode,
    source: &str,
    options: &Options,
    context: &Context,
    formatted: &mut String,
) -> Option<String> {
    let forced = options.inline_table_auto_expand && context.force_multiline;
    let mut context = context.clone();
    if context.force_multiline {
        context.force_multiline = options.inline_table_expand && !options.inline_table_auto_expand;
    }
    let context = &context;

    // A comment runs to the end of the line, so joining the lines would comment out the rest.
    // Keep one entry per line instead (TOML 1.1).
    // Only the table's own comments count, a nested array or table keeps its comments itself.
    if has_own_comment(node) {
        format_inline_table_multiline(node, source, options, context, formatted, false);
        return None;
    }

    // Check if the inline table has any ENTRY nodes (not just tokens like braces)
    let has_entries = node.children().iter().any(|c| c.kind() == ENTRY);

    if !has_entries {
        *formatted += "{}";
        return None;
    }

    if forced {
        format_inline_table_multiline(
            node,
            source,
            options,
            context,
            formatted,
            options.reorder_inline_tables,
        );
        return None;
    }

    let mut sorted_children = if options.reorder_inline_tables {
        let mut children: Vec<_> = node.children().to_vec();
        children.sort_by_cached_key(|x| x.to_string(source));
        Some(VecDeque::from(children))
    } else {
        None
    };

    let mut node_index = 0;
    for c in node.children_with_tokens() {
        match c {
            Element::Node(n) => {
                if node_index != 0 {
                    *formatted += ", ";
                }

                let sorted_child;
                let entry = if options.reorder_inline_tables {
                    sorted_child = sorted_children
                        .as_mut()
                        .and_then(|children| children.pop_front())
                        .and_then(|e| e.as_node().cloned())
                        .unwrap_or_else(|| n.clone());
                    format_entry(&sorted_child, source, options, context)
                } else {
                    format_entry(n, source, options, context)
                };

                entry.write_to(formatted, options);

                node_index += 1;
            }
            Element::Token(t) => match t.kind() {
                BRACE_START => {
                    *formatted += "{";
                    if !options.compact_inline_tables {
                        *formatted += " ";
                    }
                }
                BRACE_END => {
                    if !options.compact_inline_tables {
                        *formatted += " ";
                    }
                    *formatted += "}";
                }
                // Without comments, the table can be joined onto one line
                WHITESPACE | COMMA | NEWLINE => {}
                _ => *formatted += t.text(source),
            },
        }
    }

    None
}

/// Whether the inline table has a comment of its own,
/// directly in it or behind an entry's value.
fn has_own_comment(node: &SyntaxNode) -> bool {
    let has_comment_token = |n: &SyntaxNode| n.children().iter().any(|c| c.kind() == COMMENT);
    has_comment_token(node)
        || node.children().iter().filter_map(Element::as_node).any(|entry| {
            has_comment_token(entry)
                || entry.children().iter().filter_map(Element::as_node).any(has_comment_token)
        })
}

/// An inline table with comments, one entry or own-line comment per line.
/// Entries are not reordered, a comment belongs to the line it is on.
fn format_inline_table_multiline(
    node: &SyntaxNode,
    source: &str,
    options: &Options,
    context: &Context,
    formatted: &mut String,
    reorder: bool,
) {
    let mut inner_context = context.clone();
    inner_context.indent_level += 1;
    let indent: String = inner_context.indent(options).collect();
    let entry_count = node.children().iter().filter(|c| c.kind() == ENTRY).count();
    let mut sorted = reorder.then(|| {
        let mut entries: Vec<_> = node.children().iter().filter_map(Element::as_node).collect();
        entries.sort_by_cached_key(|n| n.text(source));
        entries.into_iter()
    });
    let recheck = options.inline_table_auto_expand && options.inline_table_expand;

    let mut entry_index = 0;
    // Whether a line break was seen since the last entry, so a comment is on its own line
    let mut on_new_line = false;
    for (elem_idx, c) in node.children_with_tokens().enumerate() {
        match c {
            Element::Node(n) => {
                *formatted += options.newline();
                *formatted += &indent;
                let n = sorted.as_mut().and_then(Iterator::next).unwrap_or(n);
                let has_comma = entry_index + 1 < entry_count;
                let entry = format_entry(n, source, options, &inner_context);
                let entry = if recheck
                    && exceeds_width(
                        indent.chars().count() + entry_prefix_chars(&entry.key, options),
                        &entry.value,
                        usize::from(has_comma)
                            + comment_chars(
                                entry
                                    .comment
                                    .as_deref()
                                    .or_else(|| trailing_comment(node, elem_idx, source)),
                            ),
                        options,
                    ) {
                    format_entry(
                        n,
                        source,
                        options,
                        &Context { force_multiline: true, ..inner_context.clone() },
                    )
                } else {
                    entry
                };
                entry.write_to(formatted, options);
                entry_index += 1;
                if has_comma {
                    *formatted += ",";
                }
                if let Some(ref c) = entry.comment {
                    *formatted += " ";
                    *formatted += c;
                }
                on_new_line = false;
            }
            Element::Token(t) => match t.kind() {
                BRACE_START => *formatted += "{",
                BRACE_END => {
                    *formatted += options.newline();
                    formatted.extend(context.indent(options));
                    *formatted += "}";
                }
                NEWLINE => on_new_line = true,
                COMMENT => {
                    if on_new_line {
                        *formatted += options.newline();
                        *formatted += &indent;
                    } else {
                        *formatted += " ";
                    }
                    *formatted += t.text(source);
                }
                _ => {}
            },
        }
    }
}

fn exceeds_width(prefix: usize, value: &str, suffix: usize, options: &Options) -> bool {
    let last = value.split('\n').count() - 1;
    value.split('\n').enumerate().any(|(idx, line)| {
        line.chars().count()
            + if idx == 0 { prefix } else { 0 }
            + if idx == last { suffix } else { 0 }
            > options.column_width
    })
}

fn entry_prefix_chars(key: &str, options: &Options) -> usize {
    key.chars().count() + if options.compact_entries { 1 } else { 3 }
}

fn comment_chars(comment: Option<&str>) -> usize {
    comment.map_or(0, |c| c.chars().count() + 1)
}

fn trailing_comment<'s>(node: &SyntaxNode, elem_idx: usize, source: &'s str) -> Option<&'s str> {
    node.children()[elem_idx + 1..]
        .iter()
        .take_while(|e| e.as_node().is_none() && e.kind() != NEWLINE)
        .find(|e| e.kind() == COMMENT)
        .map(|e| e.text(source))
}

// Check whether the array spans multiple lines in its current form.
fn is_array_multiline(node: &SyntaxNode) -> bool {
    node.descendants_with_tokens().any(|n| n.kind() == NEWLINE)
}

fn can_collapse_array(node: &SyntaxNode) -> bool {
    !node.descendants_with_tokens().any(|n| n.kind() == COMMENT)
}

fn format_array(
    node: &SyntaxNode,
    source: &str,
    options: &Options,
    context: &Context,
    formatted: &mut String,
) -> Option<String> {
    let forced =
        context.force_multiline && (options.array_auto_expand || !options.inline_table_auto_expand);
    let mut multiline = is_array_multiline(node) || forced;

    // We always try to collapse it if possible.
    if can_collapse_array(node) && options.array_auto_collapse && !forced {
        multiline = false;
    }

    // We use the same strategy as for entries, refer to [`format_root`].
    let mut skip_newlines = 0;

    // Formatted value, optional trailing comment
    // The value must not include the comma at the end.
    let mut value_group: Vec<(String, Option<String>, &SyntaxNode)> = Vec::new();
    let mut commas_group: Vec<bool> = Vec::new();

    let add_values = |value_group: &mut Vec<(String, Option<String>, &SyntaxNode)>,
                      commas_group: &mut Vec<bool>,
                      formatted: &mut String,
                      context: &Context|
     -> bool {
        let were_values = !value_group.is_empty();

        if options.reorder_arrays {
            value_group.sort_unstable_by(|x, y| x.0.cmp(&y.0));
        }

        let indent_chars = context.indent_level * options.indent_string.chars().count();
        for (has_comma, p) in commas_group.drain(0..).zip(value_group.iter_mut()) {
            if multiline
                && options.inline_table_auto_expand
                && exceeds_width(
                    indent_chars,
                    &p.0,
                    usize::from(has_comma) + comment_chars(p.1.as_deref()),
                    options,
                )
            {
                p.0.clear();
                format_value(
                    p.2,
                    source,
                    options,
                    &Context { force_multiline: true, ..context.clone() },
                    &mut p.0,
                );
            }
            if has_comma {
                p.0 += ","
            };
        }

        if !multiline {
            for (idx, (val, comment, _)) in value_group.drain(0..).enumerate() {
                debug_assert!(comment.is_none());
                if idx != 0 {
                    *formatted += " "
                }

                *formatted += &val;
            }

            return were_values;
        }

        // Fast path: when comment alignment is disabled, skip building rows.
        if !options.align_comments {
            let indent_str: String = context.indent(options).collect();
            let newline = options.newline();
            for (i, (value, comment, _)) in value_group.drain(0..).enumerate() {
                if i != 0 {
                    *formatted += newline;
                }
                *formatted += &indent_str;
                *formatted += &value;
                if let Some(c) = comment {
                    *formatted += " ";
                    *formatted += &c;
                }
            }
            return were_values;
        }

        let mut comment_count = 0;
        let rows = value_group
            .drain(0..)
            .map(|(value, comment, _)| {
                let mut row = Vec::with_capacity(5);

                row.push(context.indent(options).collect::<String>());
                row.push(value);
                if let Some(c) = comment {
                    row.push(c);
                    comment_count += 1;
                }

                row
            })
            .collect::<Vec<_>>();

        let align_comments = options.should_align_comments(comment_count);
        format_rows(
            formatted,
            if align_comments { 0..usize::MAX } else { 0..0 },
            1..usize::MAX,
            &rows,
            options.newline(),
            " ",
        );

        were_values
    };

    // Count only VALUE nodes, not all children
    let node_count = node.children().iter().filter(|c| c.kind() == VALUE).count();

    let mut inner_context = context.clone();

    if multiline {
        inner_context.indent_level += 1;
    }
    if options.inline_table_auto_expand {
        inner_context.force_multiline = false;
    }

    let mut dangling_newline_count = 0;

    let mut node_index = 0;
    for (elem_idx, c) in node.children_with_tokens().enumerate() {
        match c {
            Element::Node(n) => match n.kind() {
                VALUE => {
                    if multiline && formatted.ends_with('[') {
                        *formatted += options.newline();
                    }

                    let mut val_string = String::new();
                    let comment = format_value(n, source, options, &inner_context, &mut val_string);

                    let has_comma =
                        node_index < node_count - 1 || (multiline && options.array_trailing_comma);
                    commas_group.push(has_comma);

                    value_group.push((val_string, comment, n));
                    skip_newlines += 1;

                    node_index += 1;
                }
                _ => {
                    if cfg!(debug_assertions) {
                        unreachable!()
                    }
                }
            },
            Element::Token(t) => match t.kind() {
                BRACKET_START => {
                    *formatted += "[";
                    if !options.compact_arrays && !multiline {
                        *formatted += " ";
                    }
                }
                BRACKET_END => {
                    add_values(&mut value_group, &mut commas_group, formatted, &inner_context);

                    if multiline {
                        if !formatted.ends_with('\n') {
                            *formatted += options.newline();
                        }

                        formatted.extend(context.indent(options));
                    } else if !options.compact_arrays {
                        *formatted += " ";
                    }
                    *formatted += "]";
                }
                NEWLINE => {
                    if !multiline {
                        continue;
                    }

                    let mut newline_count = t.text(source).newline_count();

                    match dangling_newlines(t, source) {
                        Some(dnl) => {
                            dangling_newline_count += dnl;
                            continue;
                        }
                        None => {
                            newline_count += dangling_newline_count;
                            dangling_newline_count = 0;
                        }
                    }

                    if newline_count > 1 {
                        add_values(&mut value_group, &mut commas_group, formatted, &inner_context);
                        skip_newlines = 0;
                    }

                    formatted.extend(options.newlines(newline_count.saturating_sub(skip_newlines)));
                }
                COMMENT => {
                    // Check if there's a newline before this comment by looking at previous sibling
                    let newline_before = if elem_idx > 0 {
                        // Find the previous non-whitespace element
                        node.children()
                            .iter()
                            .take(elem_idx)
                            .rev()
                            .find(|e| e.kind() != WHITESPACE)
                            .is_some_and(|e| e.kind() == NEWLINE)
                    } else {
                        false
                    };

                    if !newline_before && !value_group.is_empty() {
                        // It's actually trailing comment, so we add it to the last value.
                        value_group.last_mut().unwrap().1 = Some(t.text(source).to_string());
                        continue;
                    }

                    if add_values(&mut value_group, &mut commas_group, formatted, &inner_context) {
                        *formatted += options.newline();
                        skip_newlines = 0;
                    }

                    if formatted.ends_with('[') {
                        *formatted += " ";
                        *formatted += t.text(source);
                    } else {
                        formatted.extend(inner_context.indent(options));
                        *formatted += t.text(source);
                    }
                }
                _ => {}
            },
        }
    }

    if formatted.is_empty() {
        *formatted += "[]";
    }

    None
}

fn format_table_header(
    node: &SyntaxNode,
    source: &str,
    options: &Options,
    context: &Context,
    formatted: &mut String,
) -> Option<String> {
    let mut comment = None;

    for c in node.children_with_tokens() {
        match c {
            Element::Node(n) => {
                format_key(n, source, formatted, options, context);
            }
            Element::Token(t) => match t.kind() {
                BRACKET_START | BRACKET_END => *formatted += t.text(source),
                WHITESPACE | NEWLINE => {}
                COMMENT => {
                    debug_assert!(comment.is_none());
                    comment = Some(t.text(source).to_string());
                }
                _ => *formatted += t.text(source),
            },
        }
    }

    comment
}

trait NewlineCount {
    fn newline_count(&self) -> usize;
}

impl NewlineCount for &str {
    fn newline_count(&self) -> usize {
        self.as_bytes().iter().filter(|&&b| b == b'\n').count()
    }
}

// FIXME(docs)
fn format_rows<R, S>(
    out: &mut String,
    align_range: Range<usize>,
    separator_range: Range<usize>,
    rows: &[R],
    newline: &str,
    separator: &str,
) where
    R: AsRef<[S]>,
    S: AsRef<str>,
{
    // We currently don't support vertical alignment of complex data.
    let can_align = rows.iter().flat_map(|r| r.as_ref().iter()).all(|s| !s.as_ref().contains('\n'));

    let diff_widths = |range: Range<usize>, row: &R| -> usize {
        let mut max_width = 0_usize;

        for row in rows {
            let row_len = row.as_ref().len();

            let range =
                cmp::min(range.start, row_len.saturating_sub(1))..cmp::min(range.end, row_len);

            max_width = cmp::max(
                max_width,
                row.as_ref()[range].iter().map(|s| s.as_ref().chars().count()).sum(),
            );
        }

        let row_width =
            row.as_ref()[range].iter().map(|s| s.as_ref().chars().count()).sum::<usize>();

        max_width - row_width
    };

    for (row_idx, row) in rows.iter().enumerate() {
        if row_idx != 0 {
            *out += newline;
        }

        let mut last_align_idx = 0_usize;

        for (item_idx, item) in row.as_ref().iter().enumerate() {
            if item_idx > separator_range.start
                && item_idx <= separator_range.end.saturating_add(1)
                && item_idx < row.as_ref().len()
            {
                *out += separator;
            }

            *out += item.as_ref();

            if can_align
                && align_range.start <= item_idx
                && align_range.end > item_idx
                && item_idx < row.as_ref().len() - 1
            {
                let diff = diff_widths(last_align_idx..item_idx + 1, row);
                out.extend(std::iter::repeat_n(" ", diff));
                last_align_idx = item_idx + 1;
            }
        }
    }
}

/// Special handling of blank lines.
///
/// A design decision was made in the parser that newline (LF) characters
/// and whitespace (" ", and \t) are part of separate tokens.
///
/// Generally we count the amount of blank lines by counting LF characters in a token,
/// however if any of the consecutive blank lines contain empty characters,
/// this way of counting becomes unreliable.
///
/// So we check if the newlines are followed by whitespace,
/// then newlines again, and return the count here,
/// and we can add these values up.
const fn dangling_newlines(t: &SyntaxToken, source: &str) -> Option<usize> {
    // TODO: This function needs sibling traversal which we don't have yet
    // For now, just return None to disable this optimization
    // The formatter will still work correctly, just potentially with extra newlines
    let _ = (t, source);
    None
}
