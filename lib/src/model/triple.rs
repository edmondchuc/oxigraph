use crate::model::blank_node::BlankNode;
use crate::model::literal::Literal;
use crate::model::named_node::NamedNode;
use crate::model::{BlankNodeRef, LiteralRef, NamedNodeRef};
use rio_api::model as rio;
use std::fmt;

/// The owned union of [IRIs](https://www.w3.org/TR/rdf11-concepts/#dfn-iri) and [blank nodes](https://www.w3.org/TR/rdf11-concepts/#dfn-blank-node).
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum NamedOrBlankNode {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
}

impl NamedOrBlankNode {
    pub fn is_named_node(&self) -> bool {
        self.as_ref().is_named_node()
    }

    pub fn is_blank_node(&self) -> bool {
        self.as_ref().is_blank_node()
    }

    pub fn as_ref(&self) -> NamedOrBlankNodeRef<'_> {
        match self {
            Self::NamedNode(node) => NamedOrBlankNodeRef::NamedNode(node.as_ref()),
            Self::BlankNode(node) => NamedOrBlankNodeRef::BlankNode(node.as_ref()),
        }
    }
}

impl fmt::Display for NamedOrBlankNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl From<NamedNode> for NamedOrBlankNode {
    fn from(node: NamedNode) -> Self {
        Self::NamedNode(node)
    }
}

impl From<NamedNodeRef<'_>> for NamedOrBlankNode {
    fn from(node: NamedNodeRef<'_>) -> Self {
        node.into_owned().into()
    }
}

impl From<BlankNode> for NamedOrBlankNode {
    fn from(node: BlankNode) -> Self {
        Self::BlankNode(node)
    }
}

impl From<BlankNodeRef<'_>> for NamedOrBlankNode {
    fn from(node: BlankNodeRef<'_>) -> Self {
        node.into_owned().into()
    }
}

/// The borrowed union of [IRIs](https://www.w3.org/TR/rdf11-concepts/#dfn-iri) and [blank nodes](https://www.w3.org/TR/rdf11-concepts/#dfn-blank-node).
#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub enum NamedOrBlankNodeRef<'a> {
    NamedNode(NamedNodeRef<'a>),
    BlankNode(BlankNodeRef<'a>),
}

impl<'a> NamedOrBlankNodeRef<'a> {
    pub fn is_named_node(&self) -> bool {
        match self {
            Self::NamedNode(_) => true,
            Self::BlankNode(_) => false,
        }
    }

    pub fn is_blank_node(&self) -> bool {
        match self {
            Self::NamedNode(_) => false,
            Self::BlankNode(_) => true,
        }
    }

    pub fn into_owned(self) -> NamedOrBlankNode {
        match self {
            Self::NamedNode(node) => NamedOrBlankNode::NamedNode(node.into_owned()),
            Self::BlankNode(node) => NamedOrBlankNode::BlankNode(node.into_owned()),
        }
    }
}

impl fmt::Display for NamedOrBlankNodeRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NamedNode(node) => node.fmt(f),
            Self::BlankNode(node) => node.fmt(f),
        }
    }
}

impl<'a> From<NamedNodeRef<'a>> for NamedOrBlankNodeRef<'a> {
    fn from(node: NamedNodeRef<'a>) -> Self {
        Self::NamedNode(node)
    }
}

impl<'a> From<&'a NamedNode> for NamedOrBlankNodeRef<'a> {
    fn from(node: &'a NamedNode) -> Self {
        node.as_ref().into()
    }
}

impl<'a> From<BlankNodeRef<'a>> for NamedOrBlankNodeRef<'a> {
    fn from(node: BlankNodeRef<'a>) -> Self {
        Self::BlankNode(node)
    }
}

impl<'a> From<&'a BlankNode> for NamedOrBlankNodeRef<'a> {
    fn from(node: &'a BlankNode) -> Self {
        node.as_ref().into()
    }
}

impl<'a> From<&'a NamedOrBlankNode> for NamedOrBlankNodeRef<'a> {
    fn from(node: &'a NamedOrBlankNode) -> Self {
        node.as_ref()
    }
}

impl<'a> From<NamedOrBlankNodeRef<'a>> for NamedOrBlankNode {
    fn from(node: NamedOrBlankNodeRef<'a>) -> Self {
        node.into_owned()
    }
}

impl<'a> From<NamedOrBlankNodeRef<'a>> for rio::NamedOrBlankNode<'a> {
    fn from(node: NamedOrBlankNodeRef<'a>) -> Self {
        match node {
            NamedOrBlankNodeRef::NamedNode(node) => rio::NamedNode::from(node).into(),
            NamedOrBlankNodeRef::BlankNode(node) => rio::BlankNode::from(node).into(),
        }
    }
}

/// An owned RDF [term](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-term)
/// It is the union of [IRIs](https://www.w3.org/TR/rdf11-concepts/#dfn-iri), [blank nodes](https://www.w3.org/TR/rdf11-concepts/#dfn-blank-node) and [literals](https://www.w3.org/TR/rdf11-concepts/#dfn-literal).
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum Term {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    Literal(Literal),
}

impl Term {
    pub fn is_named_node(&self) -> bool {
        self.as_ref().is_named_node()
    }

    pub fn is_blank_node(&self) -> bool {
        self.as_ref().is_blank_node()
    }

    pub fn is_literal(&self) -> bool {
        self.as_ref().is_literal()
    }

    pub fn as_ref(&self) -> TermRef<'_> {
        match self {
            Self::NamedNode(node) => TermRef::NamedNode(node.as_ref()),
            Self::BlankNode(node) => TermRef::BlankNode(node.as_ref()),
            Self::Literal(literal) => TermRef::Literal(literal.as_ref()),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl From<NamedNode> for Term {
    fn from(node: NamedNode) -> Self {
        Self::NamedNode(node)
    }
}

impl From<NamedNodeRef<'_>> for Term {
    fn from(node: NamedNodeRef<'_>) -> Self {
        node.into_owned().into()
    }
}

impl From<BlankNode> for Term {
    fn from(node: BlankNode) -> Self {
        Self::BlankNode(node)
    }
}

impl From<BlankNodeRef<'_>> for Term {
    fn from(node: BlankNodeRef<'_>) -> Self {
        node.into_owned().into()
    }
}

impl From<Literal> for Term {
    fn from(literal: Literal) -> Self {
        Self::Literal(literal)
    }
}

impl From<LiteralRef<'_>> for Term {
    fn from(literal: LiteralRef<'_>) -> Self {
        literal.into_owned().into()
    }
}

impl From<NamedOrBlankNode> for Term {
    fn from(node: NamedOrBlankNode) -> Self {
        match node {
            NamedOrBlankNode::NamedNode(node) => node.into(),
            NamedOrBlankNode::BlankNode(node) => node.into(),
        }
    }
}

impl From<NamedOrBlankNodeRef<'_>> for Term {
    fn from(node: NamedOrBlankNodeRef<'_>) -> Self {
        node.into_owned().into()
    }
}

/// A borrowed RDF [term](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-term)
/// It is the union of [IRIs](https://www.w3.org/TR/rdf11-concepts/#dfn-iri), [blank nodes](https://www.w3.org/TR/rdf11-concepts/#dfn-blank-node) and [literals](https://www.w3.org/TR/rdf11-concepts/#dfn-literal).
#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub enum TermRef<'a> {
    NamedNode(NamedNodeRef<'a>),
    BlankNode(BlankNodeRef<'a>),
    Literal(LiteralRef<'a>),
}

impl<'a> TermRef<'a> {
    pub fn is_named_node(&self) -> bool {
        match self {
            Self::NamedNode(_) => true,
            _ => false,
        }
    }

    pub fn is_blank_node(&self) -> bool {
        match self {
            Self::BlankNode(_) => true,
            _ => false,
        }
    }

    pub fn is_literal(&self) -> bool {
        match self {
            Self::Literal(_) => true,
            _ => false,
        }
    }

    pub fn into_owned(self) -> Term {
        match self {
            Self::NamedNode(node) => Term::NamedNode(node.into_owned()),
            Self::BlankNode(node) => Term::BlankNode(node.into_owned()),
            Self::Literal(literal) => Term::Literal(literal.into_owned()),
        }
    }
}

impl fmt::Display for TermRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NamedNode(node) => node.fmt(f),
            Self::BlankNode(node) => node.fmt(f),
            Self::Literal(node) => node.fmt(f),
        }
    }
}

impl<'a> From<NamedNodeRef<'a>> for TermRef<'a> {
    fn from(node: NamedNodeRef<'a>) -> Self {
        Self::NamedNode(node)
    }
}

impl<'a> From<&'a NamedNode> for TermRef<'a> {
    fn from(node: &'a NamedNode) -> Self {
        node.as_ref().into()
    }
}

impl<'a> From<BlankNodeRef<'a>> for TermRef<'a> {
    fn from(node: BlankNodeRef<'a>) -> Self {
        Self::BlankNode(node)
    }
}

impl<'a> From<&'a BlankNode> for TermRef<'a> {
    fn from(node: &'a BlankNode) -> Self {
        node.as_ref().into()
    }
}

impl<'a> From<LiteralRef<'a>> for TermRef<'a> {
    fn from(literal: LiteralRef<'a>) -> Self {
        Self::Literal(literal)
    }
}

impl<'a> From<&'a Literal> for TermRef<'a> {
    fn from(literal: &'a Literal) -> Self {
        literal.as_ref().into()
    }
}

impl<'a> From<NamedOrBlankNodeRef<'a>> for TermRef<'a> {
    fn from(node: NamedOrBlankNodeRef<'a>) -> Self {
        match node {
            NamedOrBlankNodeRef::NamedNode(node) => node.into(),
            NamedOrBlankNodeRef::BlankNode(node) => node.into(),
        }
    }
}

impl<'a> From<&'a NamedOrBlankNode> for TermRef<'a> {
    fn from(node: &'a NamedOrBlankNode) -> Self {
        node.as_ref().into()
    }
}

impl<'a> From<&'a Term> for TermRef<'a> {
    fn from(node: &'a Term) -> Self {
        node.as_ref()
    }
}

impl<'a> From<TermRef<'a>> for Term {
    fn from(node: TermRef<'a>) -> Self {
        node.into_owned()
    }
}

impl<'a> From<TermRef<'a>> for rio::Term<'a> {
    fn from(node: TermRef<'a>) -> Self {
        match node {
            TermRef::NamedNode(node) => rio::NamedNode::from(node).into(),
            TermRef::BlankNode(node) => rio::BlankNode::from(node).into(),
            TermRef::Literal(node) => rio::Literal::from(node).into(),
        }
    }
}

/// An owned [RDF triple](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-triple)
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Triple {
    /// The [subject](https://www.w3.org/TR/rdf11-concepts/#dfn-subject) of this triple
    pub subject: NamedOrBlankNode,

    /// The [predicate](https://www.w3.org/TR/rdf11-concepts/#dfn-predicate) of this triple
    pub predicate: NamedNode,

    /// The [object](https://www.w3.org/TR/rdf11-concepts/#dfn-object) of this triple
    pub object: Term,
}

impl Triple {
    /// Builds an RDF [triple](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-triple)
    pub fn new(
        subject: impl Into<NamedOrBlankNode>,
        predicate: impl Into<NamedNode>,
        object: impl Into<Term>,
    ) -> Self {
        Self {
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
        }
    }

    #[deprecated(note = "Use directly the `subject` field")]
    pub const fn subject(&self) -> &NamedOrBlankNode {
        &self.subject
    }

    #[deprecated(note = "Use directly the `subject` field")]
    pub fn subject_owned(self) -> NamedOrBlankNode {
        self.subject
    }

    #[deprecated(note = "Use directly the `predicate` field")]
    pub const fn predicate(&self) -> &NamedNode {
        &self.predicate
    }

    #[deprecated(note = "Use directly the `predicate` field")]
    pub fn predicate_owned(self) -> NamedNode {
        self.predicate
    }

    #[deprecated(note = "Use directly the `object` field")]
    pub const fn object(&self) -> &Term {
        &self.object
    }

    #[deprecated(note = "Use directly the `object` field")]
    pub fn object_owned(self) -> Term {
        self.object
    }

    /// Encodes that this triple is in a [RDF dataset](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-dataset)
    pub fn in_graph(self, graph_name: impl Into<GraphName>) -> Quad {
        Quad {
            subject: self.subject,
            predicate: self.predicate,
            object: self.object,
            graph_name: graph_name.into(),
        }
    }

    pub fn as_ref(&self) -> TripleRef<'_> {
        TripleRef {
            subject: self.subject.as_ref(),
            predicate: self.predicate.as_ref(),
            object: self.object.as_ref(),
        }
    }
}

impl fmt::Display for Triple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

/// A borrowed [RDF triple](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-triple)
#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub struct TripleRef<'a> {
    /// The [subject](https://www.w3.org/TR/rdf11-concepts/#dfn-subject) of this triple
    pub subject: NamedOrBlankNodeRef<'a>,

    /// The [predicate](https://www.w3.org/TR/rdf11-concepts/#dfn-predicate) of this triple
    pub predicate: NamedNodeRef<'a>,

    /// The [object](https://www.w3.org/TR/rdf11-concepts/#dfn-object) of this triple
    pub object: TermRef<'a>,
}

impl<'a> TripleRef<'a> {
    /// Builds an RDF [triple](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-triple)
    pub fn new(
        subject: impl Into<NamedOrBlankNodeRef<'a>>,
        predicate: impl Into<NamedNodeRef<'a>>,
        object: impl Into<TermRef<'a>>,
    ) -> Self {
        Self {
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
        }
    }

    /// Encodes that this triple is in a [RDF dataset](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-dataset)
    pub fn in_graph(self, graph_name: impl Into<GraphNameRef<'a>>) -> QuadRef<'a> {
        QuadRef {
            subject: self.subject,
            predicate: self.predicate,
            object: self.object,
            graph_name: graph_name.into(),
        }
    }

    pub fn into_owned(self) -> Triple {
        Triple {
            subject: self.subject.into_owned(),
            predicate: self.predicate.into_owned(),
            object: self.object.into_owned(),
        }
    }
}

impl fmt::Display for TripleRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        rio::Triple::from(*self).fmt(f)
    }
}

impl<'a> From<&'a Triple> for TripleRef<'a> {
    fn from(triple: &'a Triple) -> Self {
        triple.as_ref()
    }
}

impl<'a> From<TripleRef<'a>> for Triple {
    fn from(triple: TripleRef<'a>) -> Self {
        triple.into_owned()
    }
}

impl<'a> From<TripleRef<'a>> for rio::Triple<'a> {
    fn from(triple: TripleRef<'a>) -> Self {
        rio::Triple {
            subject: triple.subject.into(),
            predicate: triple.predicate.into(),
            object: triple.object.into(),
        }
    }
}

/// A possible owned graph name.
/// It is the union of [IRIs](https://www.w3.org/TR/rdf11-concepts/#dfn-iri), [blank nodes](https://www.w3.org/TR/rdf11-concepts/#dfn-blank-node) and the [default graph name](https://www.w3.org/TR/rdf11-concepts/#dfn-default-graph).
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum GraphName {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    DefaultGraph,
}

impl GraphName {
    pub fn is_named_node(&self) -> bool {
        self.as_ref().is_named_node()
    }

    pub fn is_blank_node(&self) -> bool {
        self.as_ref().is_blank_node()
    }

    pub fn is_default_graph(&self) -> bool {
        self.as_ref().is_default_graph()
    }

    pub fn as_ref(&self) -> GraphNameRef<'_> {
        match self {
            Self::NamedNode(node) => GraphNameRef::NamedNode(node.as_ref()),
            Self::BlankNode(node) => GraphNameRef::BlankNode(node.as_ref()),
            Self::DefaultGraph => GraphNameRef::DefaultGraph,
        }
    }
}

impl fmt::Display for GraphName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl From<NamedNode> for GraphName {
    fn from(node: NamedNode) -> Self {
        GraphName::NamedNode(node)
    }
}

impl From<NamedNodeRef<'_>> for GraphName {
    fn from(node: NamedNodeRef<'_>) -> Self {
        node.into_owned().into()
    }
}

impl From<BlankNode> for GraphName {
    fn from(node: BlankNode) -> Self {
        GraphName::BlankNode(node)
    }
}

impl From<BlankNodeRef<'_>> for GraphName {
    fn from(node: BlankNodeRef<'_>) -> Self {
        node.into_owned().into()
    }
}

impl From<NamedOrBlankNode> for GraphName {
    fn from(node: NamedOrBlankNode) -> Self {
        match node {
            NamedOrBlankNode::NamedNode(node) => node.into(),
            NamedOrBlankNode::BlankNode(node) => node.into(),
        }
    }
}

impl From<NamedOrBlankNodeRef<'_>> for GraphName {
    fn from(node: NamedOrBlankNodeRef<'_>) -> Self {
        node.into_owned().into()
    }
}

impl From<Option<NamedOrBlankNode>> for GraphName {
    fn from(name: Option<NamedOrBlankNode>) -> Self {
        if let Some(node) = name {
            node.into()
        } else {
            GraphName::DefaultGraph
        }
    }
}

impl From<GraphName> for Option<NamedOrBlankNode> {
    fn from(name: GraphName) -> Self {
        match name {
            GraphName::NamedNode(node) => Some(node.into()),
            GraphName::BlankNode(node) => Some(node.into()),
            GraphName::DefaultGraph => None,
        }
    }
}

/// A possible borrowed graph name.
/// It is the union of [IRIs](https://www.w3.org/TR/rdf11-concepts/#dfn-iri), [blank nodes](https://www.w3.org/TR/rdf11-concepts/#dfn-blank-node) and the [default graph name](https://www.w3.org/TR/rdf11-concepts/#dfn-default-graph).
#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub enum GraphNameRef<'a> {
    NamedNode(NamedNodeRef<'a>),
    BlankNode(BlankNodeRef<'a>),
    DefaultGraph,
}

impl<'a> GraphNameRef<'a> {
    pub fn is_named_node(&self) -> bool {
        match self {
            GraphNameRef::NamedNode(_) => true,
            _ => false,
        }
    }

    pub fn is_blank_node(&self) -> bool {
        match self {
            GraphNameRef::BlankNode(_) => true,
            _ => false,
        }
    }

    pub fn is_default_graph(&self) -> bool {
        match self {
            GraphNameRef::DefaultGraph => true,
            _ => false,
        }
    }

    pub fn into_owned(self) -> GraphName {
        match self {
            Self::NamedNode(node) => GraphName::NamedNode(node.into_owned()),
            Self::BlankNode(node) => GraphName::BlankNode(node.into_owned()),
            Self::DefaultGraph => GraphName::DefaultGraph,
        }
    }
}

impl fmt::Display for GraphNameRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NamedNode(node) => node.fmt(f),
            Self::BlankNode(node) => node.fmt(f),
            Self::DefaultGraph => write!(f, "DEFAULT"),
        }
    }
}

impl<'a> From<NamedNodeRef<'a>> for GraphNameRef<'a> {
    fn from(node: NamedNodeRef<'a>) -> Self {
        Self::NamedNode(node)
    }
}

impl<'a> From<&'a NamedNode> for GraphNameRef<'a> {
    fn from(node: &'a NamedNode) -> Self {
        node.as_ref().into()
    }
}

impl<'a> From<BlankNodeRef<'a>> for GraphNameRef<'a> {
    fn from(node: BlankNodeRef<'a>) -> Self {
        Self::BlankNode(node)
    }
}

impl<'a> From<&'a BlankNode> for GraphNameRef<'a> {
    fn from(node: &'a BlankNode) -> Self {
        node.as_ref().into()
    }
}

impl<'a> From<NamedOrBlankNodeRef<'a>> for GraphNameRef<'a> {
    fn from(node: NamedOrBlankNodeRef<'a>) -> Self {
        match node {
            NamedOrBlankNodeRef::NamedNode(node) => node.into(),
            NamedOrBlankNodeRef::BlankNode(node) => node.into(),
        }
    }
}

impl<'a> From<&'a NamedOrBlankNode> for GraphNameRef<'a> {
    fn from(node: &'a NamedOrBlankNode) -> Self {
        node.as_ref().into()
    }
}

impl<'a> From<&'a GraphName> for GraphNameRef<'a> {
    fn from(node: &'a GraphName) -> Self {
        node.as_ref()
    }
}

impl<'a> From<GraphNameRef<'a>> for GraphName {
    fn from(node: GraphNameRef<'a>) -> Self {
        node.into_owned()
    }
}

impl<'a> From<Option<NamedOrBlankNodeRef<'a>>> for GraphNameRef<'a> {
    fn from(name: Option<NamedOrBlankNodeRef<'a>>) -> Self {
        if let Some(node) = name {
            node.into()
        } else {
            GraphNameRef::DefaultGraph
        }
    }
}

impl<'a> From<GraphNameRef<'a>> for Option<NamedOrBlankNodeRef<'a>> {
    fn from(name: GraphNameRef<'a>) -> Self {
        match name {
            GraphNameRef::NamedNode(node) => Some(node.into()),
            GraphNameRef::BlankNode(node) => Some(node.into()),
            GraphNameRef::DefaultGraph => None,
        }
    }
}

impl<'a> From<GraphNameRef<'a>> for Option<rio::NamedOrBlankNode<'a>> {
    fn from(name: GraphNameRef<'a>) -> Self {
        match name {
            GraphNameRef::NamedNode(node) => Some(rio::NamedNode::from(node).into()),
            GraphNameRef::BlankNode(node) => Some(rio::BlankNode::from(node).into()),
            GraphNameRef::DefaultGraph => None,
        }
    }
}

/// An owned [triple](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-triple) in a [RDF dataset](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-dataset)
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Quad {
    /// The [subject](https://www.w3.org/TR/rdf11-concepts/#dfn-subject) of this triple
    pub subject: NamedOrBlankNode,

    /// The [predicate](https://www.w3.org/TR/rdf11-concepts/#dfn-predicate) of this triple
    pub predicate: NamedNode,

    /// The [object](https://www.w3.org/TR/rdf11-concepts/#dfn-object) of this triple
    pub object: Term,

    /// The name of the RDF [graph](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-graph) in which the triple is.
    pub graph_name: GraphName,
}

impl Quad {
    /// Builds an RDF [triple](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-triple) in a [RDF dataset](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-dataset)
    pub fn new(
        subject: impl Into<NamedOrBlankNode>,
        predicate: impl Into<NamedNode>,
        object: impl Into<Term>,
        graph_name: impl Into<GraphName>,
    ) -> Self {
        Self {
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
            graph_name: graph_name.into(),
        }
    }

    #[deprecated(note = "Use directly the `subject` field")]
    pub const fn subject(&self) -> &NamedOrBlankNode {
        &self.subject
    }

    #[deprecated(note = "Use directly the `subject` field")]
    pub fn subject_owned(self) -> NamedOrBlankNode {
        self.subject
    }

    #[deprecated(note = "Use directly the `predicate` field")]
    pub const fn predicate(&self) -> &NamedNode {
        &self.predicate
    }

    #[deprecated(note = "Use directly the `predicate` field")]
    pub fn predicate_owned(self) -> NamedNode {
        self.predicate
    }

    #[deprecated(note = "Use directly the `object` field")]
    pub const fn object(&self) -> &Term {
        &self.object
    }

    #[deprecated(note = "Use directly the `object` field")]
    pub fn object_owned(self) -> Term {
        self.object
    }

    #[deprecated(note = "Use directly the `graph_name` field")]
    pub const fn graph_name(&self) -> &GraphName {
        &self.graph_name
    }

    #[deprecated(note = "Use directly the `graph_name` field")]
    pub fn graph_name_owned(self) -> GraphName {
        self.graph_name
    }

    #[deprecated(note = "Use `Triple::from` instead")]
    pub fn into_triple(self) -> Triple {
        Triple::new(self.subject, self.predicate, self.object)
    }

    #[deprecated(note = "Use directly the struct fields")]
    pub fn destruct(self) -> (NamedOrBlankNode, NamedNode, Term, GraphName) {
        (self.subject, self.predicate, self.object, self.graph_name)
    }

    pub fn as_ref(&self) -> QuadRef<'_> {
        QuadRef {
            subject: self.subject.as_ref(),
            predicate: self.predicate.as_ref(),
            object: self.object.as_ref(),
            graph_name: self.graph_name.as_ref(),
        }
    }
}

impl fmt::Display for Quad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl From<Quad> for Triple {
    fn from(quad: Quad) -> Self {
        Self {
            subject: quad.subject,
            predicate: quad.predicate,
            object: quad.object,
        }
    }
}

/// A borrowed [triple](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-triple) in a [RDF dataset](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-dataset)
#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub struct QuadRef<'a> {
    /// The [subject](https://www.w3.org/TR/rdf11-concepts/#dfn-subject) of this triple
    pub subject: NamedOrBlankNodeRef<'a>,

    /// The [predicate](https://www.w3.org/TR/rdf11-concepts/#dfn-predicate) of this triple
    pub predicate: NamedNodeRef<'a>,

    /// The [object](https://www.w3.org/TR/rdf11-concepts/#dfn-object) of this triple
    pub object: TermRef<'a>,

    /// The name of the RDF [graph](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-graph) in which the triple is.
    pub graph_name: GraphNameRef<'a>,
}

impl<'a> QuadRef<'a> {
    /// Builds an RDF [triple](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-triple) in a [RDF dataset](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-dataset)
    pub fn new(
        subject: impl Into<NamedOrBlankNodeRef<'a>>,
        predicate: impl Into<NamedNodeRef<'a>>,
        object: impl Into<TermRef<'a>>,
        graph_name: impl Into<GraphNameRef<'a>>,
    ) -> Self {
        Self {
            subject: subject.into(),
            predicate: predicate.into(),
            object: object.into(),
            graph_name: graph_name.into(),
        }
    }

    pub fn into_owned(self) -> Quad {
        Quad {
            subject: self.subject.into_owned(),
            predicate: self.predicate.into_owned(),
            object: self.object.into_owned(),
            graph_name: self.graph_name.into_owned(),
        }
    }
}

impl fmt::Display for QuadRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        rio::Quad::from(*self).fmt(f)
    }
}

impl<'a> From<QuadRef<'a>> for TripleRef<'a> {
    fn from(quad: QuadRef<'a>) -> Self {
        Self {
            subject: quad.subject,
            predicate: quad.predicate,
            object: quad.object,
        }
    }
}

impl<'a> From<&'a Quad> for QuadRef<'a> {
    fn from(quad: &'a Quad) -> Self {
        quad.as_ref()
    }
}

impl<'a> From<QuadRef<'a>> for Quad {
    fn from(quad: QuadRef<'a>) -> Self {
        quad.into_owned()
    }
}

impl<'a> From<QuadRef<'a>> for rio::Quad<'a> {
    fn from(quad: QuadRef<'a>) -> Self {
        rio::Quad {
            subject: quad.subject.into(),
            predicate: quad.predicate.into(),
            object: quad.object.into(),
            graph_name: quad.graph_name.into(),
        }
    }
}
