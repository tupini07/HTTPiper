pub type Program = Vec<ProgramStatement>;
pub type SubstitutionableContent = Vec<SubstitutionContentParts>;

#[derive(Debug)]
pub enum RequestResponse {
    Request,
    Response,
}

#[derive(Debug)]
pub enum RequestParts {
    Body,
    Headers,
}

#[derive(Debug)]
pub enum SubstitutionParts {
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
    root: SubstitutionParts,
    commands: Vec<String>,
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
    key: String,
    value: SubstitutionableContent,
}

#[derive(Debug)]
pub enum BodyValues {
    Array(Vec<BodyValues>),
    Object(Vec<(String, BodyValues)>),
    Terminal(SubstitutionableContent),
}

#[derive(Debug)]
pub struct RequestDefinition {
    name: String,
    method: String,
    url: SubstitutionableContent,
    headers: Vec<SingleHeader>,
    body: BodyValues,
}
