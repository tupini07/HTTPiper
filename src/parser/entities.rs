pub type Program = Vec<ProgramStatement>;
pub type SubstitutionableContent = Vec<SubstitutionContentParts>;

#[derive(Debug, PartialEq)]
pub enum RequestResponse {
    Request,
    Response,
}

#[derive(Debug, PartialEq)]
pub enum RequestParts {
    Body,
    Headers,
}

#[derive(Debug, PartialEq)]
pub enum SubstitutionRoot {
    Empty,
    VariableReference(String),
    NoSubtitution(String),
    RequestReference {
        name: String,
        req_resp: RequestResponse,
        part: RequestParts,
    },
}

#[derive(Debug)]
pub struct SubstitutionDetails {
    pub root: SubstitutionRoot,
    pub commands: Vec<String>,
}

#[derive(Debug)]
pub enum SubstitutionContentParts {
    NoSobstitution(String),
    Substitution(SubstitutionDetails),
}

#[derive(Debug)]
pub enum ProgramStatement {
    ImportFileName(String),
    VariableAssignment {
        name: String,
        value: SubstitutionableContent,
    },
    RequestDefinition(RequestDefinition),
}

#[derive(Debug)]
pub struct SingleHeader {
    pub key: String,
    pub value: SubstitutionableContent,
}

#[derive(Debug)]
pub enum BodyValues {
    Array(Vec<BodyValues>),
    Object(Vec<(String, BodyValues)>),
    Terminal(SubstitutionableContent),
}

#[derive(Debug)]
pub struct RequestDefinition {
    pub name: String,
    pub method: String,
    pub url: SubstitutionableContent,
    pub headers: Vec<SingleHeader>,
    pub body: BodyValues,
}
