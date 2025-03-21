pub const QUERY_PROJECT: &str = r#"
query($organization: String!, $projectNumber: Int!) {
  organization(login: $organization) {
    projectV2(number: $projectNumber) {
      id
      title
      number
      url
      createdAt
      updatedAt
    }
  }
}
"#;
