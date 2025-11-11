use crate::AppError;

/*
 * TODO
 *
 * Ask the user for jira url (mandatory) and project key (optional), then
 * create a config file in the home directory with the provided information.
 *
    This is the conf to be created:
    {
    "jira_url": "USER_PROVIDED_URL",
    "project_key": "USER_PROVIDED_PROJECT_KEY",
    "properties": {},
    "profiles": {
        "default": {
                "fields": {
                }
        }
    }
    }

    Before creating the file, check if a config file already exists. If it does, prompt the user
    to confirm overwriting it.

*/

pub async fn handle_command() -> Result<(), AppError> {
    todo!();
}
