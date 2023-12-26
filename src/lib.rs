use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

#[allow(dead_code)]
pub type OpenApiV2 = OpenApi;
pub type Schemas = HashMap<String, SchemaOrReference>;

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct OpenApi {
    pub swagger: String,
    pub info: Info,
    pub host: Option<String>,
    pub base_path: Option<String>,
    pub schemes: Option<Vec<String>>,
    pub consumes: Option<Vec<String>>,
    pub produces: Option<Vec<String>>,
    pub paths: HashMap<String, Path>,
    pub definitions: Option<Schemas>,
    pub parameters: Option<HashMap<String, Parameter>>,
    pub responses: Option<HashMap<String, Response>>,
    pub tags: Option<Vec<Tag>>,
    pub security: Option<Vec<HashMap<String, Vec<String>>>>,
    pub external_docs: Option<ExternalDocumentation>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Info {
    pub title: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Path {
    #[serde(rename="$ref")]
    pub ref_path: Option<String>,
    pub get: Option<Operation>,
    pub put: Option<Operation>,
    pub post: Option<Operation>,
    pub delete: Option<Operation>,
    pub options: Option<Operation>,
    pub head: Option<Operation>,
    pub patch: Option<Operation>,
    pub parameters: Option<Vec<ParameterOrReference>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Operation {
    pub tags: Option<Vec<String>>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub external_docs: Option<ExternalDocumentation>,
    pub operation_id: Option<String>,
    pub consumes: Option<Vec<String>>,
    pub produces: Option<Vec<String>>,
    pub parameters: Option<Vec<ParameterOrReference>>,
    pub responses: HashMap<String, ResponseOrReference>,
    pub schemes: Option<Vec<String>>,
    pub deprecated: Option<bool>,
    pub security: Option<Vec<HashMap<String, Vec<String>>>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Reference {
    #[serde(rename="$ref")]
    pub ref_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
#[serde(untagged)]
pub enum ParameterOrReference {
    Parameter(Parameter),
    Reference(Reference),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
#[serde(untagged)]
pub enum ResponseOrReference {
    Response(Response),
    Reference(Reference),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
#[serde(untagged)]
pub enum SchemaOrReference {
    Schema(Schema),
    Reference(Reference),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Parameter {
    pub name: String,
    #[serde(rename="in")]
    pub in_: ParameterType,
    pub description: Option<String>,
    pub required: Option<bool>,
    #[serde(rename="type")]
    pub type_: Option<PrimitiveType>,
    pub schema: Option<Schema>,
    pub format: Option<TypeFormat>,
    pub allow_empty_value: Option<bool>,
    pub items: Option<Schemas>,
    pub collection_format: Option<String>,
    pub default: Option<Value>,
    pub maximum: Option<u64>,
    pub exclusize_maximum: Option<u64>,
    pub minimum: Option<u64>,
    pub exclusive_minimum: Option<u64>,
    pub max_length: Option<u64>,
    pub min_length: Option<u64>,
    pub pattern: Option<String>,
    pub max_items: Option<u64>,
    pub min_items: Option<u64>,
    pub unique_items: Option<bool>,
    #[serde(rename="enum")]
    pub enum_: Option<Vec<Value>>,
    pub multiple_of: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Response {
    pub description: String,
    pub schema: Option<Schema>,
    pub headers: Option<Schemas>,
    pub examples: Option<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Schema {
    pub format: Option<TypeFormat>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub default: Option<Value>,
    pub multiple_of: Option<u64>,
    pub maximum: Option<u64>,
    pub exclusize_maximum: Option<u64>,
    pub minimum: Option<u64>,
    pub exclusive_minimum: Option<u64>,
    pub max_length: Option<u64>,
    pub min_length: Option<u64>,
    pub pattern: Option<String>,
    pub max_items: Option<u64>,
    pub min_items: Option<u64>,
    pub unique_items: Option<bool>,
    pub max_properties: Option<u64>,
    pub min_properties: Option<u64>,
    pub required: Option<Vec<String>>,
    #[serde(rename="enum")]
    pub enum_: Option<Vec<Value>>,
    #[serde(rename="type")]
    pub type_: Option<StringOrArray>,
    pub items: Option<Box<Items>>,
    pub all_of: Option<Box<Schema>>,
    pub properties: Option<Schemas>,
    pub additional_properties: Option<Box<AdditionalProperties>>,
    pub discriminator: Option<String>,
    pub read_only: Option<bool>,
    pub xml: Option<Xml>,
    pub external_docs: Option<ExternalDocumentation>,
    pub example: Option<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
#[serde(untagged)]
pub enum Items {
    Schema(Schema),
    Schemas(Vec<Schema>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
#[serde(untagged)]
pub enum StringOrArray {
    String(PrimitiveType),
    Array(Vec<PrimitiveType>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
#[serde(untagged)]
pub enum AdditionalProperties {
    Bool(bool),
    Schema(Schema),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub enum PrimitiveType {
    Null,
    Boolean,
    Object,
    Array,
    Number,
    String,
    Integer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub enum TypeFormat {
    Int32,
    Int64,
    Float,
    Double,
    Byte,
    Binary,
    Date,
    #[serde(rename="date-time")]
    DateTime,
    Password,
    Uri,
    Email,
    Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub enum ParameterType {
    Path,
    Query,
    Header,
    Body,
    FormData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Xml {}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Tag {
    pub name: String,
    pub description: Option<String>,
    pub external_docs: Option<ExternalDocumentation>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ExternalDocumentation {
    pub description: String,
    pub url: String,
}

