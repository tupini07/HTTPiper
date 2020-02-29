pub type Program = Vec<ProgramStatement>;
pub type SubstitutionableContent = Vec<SubstitutionContentParts>;

/// These denote the different parts of a RequestPart
#[derive(Debug, PartialEq)]
pub enum RequestResponse {
    Request,
    Response,
}

/// These denote the different parts of a RequestResponse
#[derive(Debug, PartialEq)]
pub enum RequestParts {
    Body,
    Headers,
}

/// This is the first element in a substitution expression.
/// It can either be _empty_, pointing to a variable, or pointing to a request.
#[derive(Debug, PartialEq)]
pub enum SubstitutionRoot {
    Empty,
    VariableReference(String),
    RequestReference {
        name: String,
        req_resp: RequestResponse,
        part: RequestParts,
    },
}

/// A single substitution can have a root followed by zero or more
/// piped commands
#[derive(Debug)]
pub struct SubstitutionDetails {
    pub root: SubstitutionRoot,
    pub commands: Vec<String>,
}

/// Any content/value can be an composed of multiple parts, each of
/// which can either be a _substitution_ or plain content (so no substitution
/// needed).
#[derive(Debug)]
pub enum SubstitutionContentParts {
    NoSobstitution(String),
    Substitution(SubstitutionDetails),
}

/// A program is composed a multiple kinds of statements:
/// - Import statement
/// - Variable assignment
/// - Request definition
#[derive(Debug)]
pub enum ProgramStatement {
    ImportFileName(String),
    VariableAssignment {
        name: String,
        value: SubstitutionableContent,
    },
    RequestDefinition(RequestDefinition),
}

/// Represents a single key value pair, which is a header
/// in a request
#[derive(Debug)]
pub struct SingleHeader {
    pub key: String,
    pub value: SubstitutionableContent,
}

/// This basically represents a JSON object which is the body of
/// a request. In a body, any _value_ can be substitutable content
#[derive(Debug)]
pub enum BodyValues {
    Array(Vec<BodyValues>),
    Object(Vec<(String, BodyValues)>),
    Terminal(SubstitutionableContent),
}

/// A struct which represents a single request definition
#[derive(Debug)]
pub struct RequestDefinition {
    pub name: String,
    pub method: String,
    pub url: SubstitutionableContent,
    pub headers: Vec<SingleHeader>,
    pub body: BodyValues,
}
