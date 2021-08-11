use anyhow::Result;

use crate::Client;

pub struct CustomFields {
    client: Client,
}

impl CustomFields {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CustomFields { client }
    }

    /**
     * Get an employee's custom fields.
     *
     * This function performs a `GET` to the `/v1/employees/{employee_id}/custom_fields` endpoint.
     *
     * Returns a list of the employee's custom fields.
     */
    pub async fn get_employees_employee_id_custom_fields(
        &self,
        employee_id: &str,
    ) -> Result<crate::types::GetEmployeesEmployeeIdCustomFieldsResponse> {
        let url = format!(
            "/v1/employees/{}/custom_fields",
            crate::progenitor_support::encode_path(&employee_id.to_string()),
        );

        self.client.get(&url).await
    }

    /**
     * Get the custom fields of a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id}/custom_fields` endpoint.
     *
     * Returns a list of the custom fields of the company. Useful when you need to know the schema of custom fields for an entire company
     */
    pub async fn get_company_custom_fields(
        &self,
        company_id: &str,
    ) -> Result<crate::types::GetCompanyCustomFieldsResponse> {
        let url = format!(
            "/v1/companies/{}/custom_fields",
            crate::progenitor_support::encode_path(&company_id.to_string()),
        );

        self.client.get(&url).await
    }
}
