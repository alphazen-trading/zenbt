import requests
import json
from requests.auth import HTTPBasicAuth
from rich import print
from pydantic import BaseModel, ConfigDict

from typing import Optional


class TokenManager(BaseModel):
    admin_user: str = "admin"
    admin_password: str = "pass"
    grafana_url: str = "http://localhost:8000"
    service_account_name: str = "my_account"
    service_account_token_name: str = "my_token"

    auth: HTTPBasicAuth = HTTPBasicAuth("", "")
    headers: dict = {}

    model_config = ConfigDict(arbitrary_types_allowed=True)

    def create_service_account(self):
        """
        Create a new service account.

        :param name: Desired name for the service account
        :param role: Role for the service account ('Viewer', 'Editor', or 'Admin')
        :return: Dictionary containing service account details or None if creation failed
        """
        payload = {"name": self.service_account_name, "role": "Admin"}
        response = requests.post(
            f"{self.grafana_url}/api/serviceaccounts",
            headers=self.headers,
            auth=self.auth,
            data=json.dumps(payload),
        )
        if response.status_code == 201:
            return response.json()
        else:
            print(
                f"Failed to create service account. Status code: {response.status_code}, Response: {response.text}"
            )
            return None

    def generate_service_account_token(self, service_account_id, token_name):
        """
        Generate a token for an existing service account.

        :param service_account_id: ID of the service account
        :param token_name: Desired name for the token
        :return: Dictionary containing token details or None if generation failed
        """
        payload = {"name": token_name}
        response = requests.post(
            f"{self.grafana_url}/api/serviceaccounts/{service_account_id}/tokens",
            headers=self.headers,
            auth=self.auth,
            data=json.dumps(payload),
        )
        if response.status_code == 200:
            return response.json()
        else:
            print(
                f"Failed to create service account token. Status code: {response.status_code}, Response: {response.text}"
            )
            return None

    def get_service_account_id_by_name(self):
        """
        Retrieve the ID of a service account by its name.

        :param name: Name of the service account
        :return: Service account ID if found, None otherwise
        """
        response = requests.get(
            f"{self.grafana_url}/api/serviceaccounts/search",
            headers=self.headers,
            auth=self.auth,
            params={"query": self.service_account_name},
        )
        if response.status_code == 200:
            accounts = response.json().get("serviceAccounts", [])
            for account in accounts:
                if account["name"] == self.service_account_name:
                    return account["id"]
            print(f"Service account with name '{self.service_account_name}' not found.")
            return None
        else:
            print(
                f"Failed to retrieve service accounts. Status code: {response.status_code}, Response: {response.text}"
            )
            return None

    def delete_service_account(self, service_account_id):
        """
        Delete an existing service account by its ID.

        :param service_account_id: ID of the service account to be deleted
        :return: True if deletion was successful, False otherwise
        """
        response = requests.delete(
            f"{self.grafana_url}/api/serviceaccounts/{service_account_id}",
            headers=self.headers,
            auth=self.auth,
        )
        if response.status_code == 200:
            print(f"Service Account with ID {service_account_id} deleted successfully.")
            return True
        else:
            print(
                f"Failed to delete service account. Status code: {response.status_code}, Response: {response.text}"
            )
            return False

    def delete_service_account_by_name(self):
        """
        Delete a service account by its name.

        :param name: Name of the service account to be deleted
        :return: True if deletion was successful, False otherwise
        """
        service_account_id = self.get_service_account_id_by_name()
        if service_account_id is None:
            return False

        response = requests.delete(
            f"{self.grafana_url}/api/serviceaccounts/{service_account_id}",
            headers=self.headers,
            auth=self.auth,
        )
        if response.status_code == 200:
            print(
                f"Service account '{self.service_account_name}' deleted successfully."
            )
            return True
        else:
            print(
                f"Failed to delete service account '{self.service_account_name}'. Status code: {response.status_code}, Response: {response.text}"
            )
            return False

    def get_key(self):
        self.auth = HTTPBasicAuth(self.admin_user, self.admin_password)
        self.headers = {"Content-Type": "application/json"}
        self.delete_service_account_by_name()

        service_account = self.create_service_account()

        if service_account:
            # Generate a token for the newly created service account
            token = manager.generate_service_account_token(
                service_account_id=service_account["id"],
                token_name=self.service_account_token_name,
            )

            if token:
                print("Service Account Token Created")
                print(token)


# Initialize the manager with Grafana server details and admin credentials
manager = TokenManager()
# print("ASD")
manager.get_key()


class GrafanaDashboardUploader:
    def __init__(self, grafana_url: str, username: str, password: str):
        """
        Initialize the uploader with Grafana server details and credentials.

        :param grafana_url: URL of the Grafana server (e.g., 'http://localhost:8000')
        :param username: Grafana username
        :param password: Grafana password
        """
        self.grafana_url = grafana_url.rstrip("/")
        self.auth = HTTPBasicAuth(username, password)
        self.headers = {"Content-Type": "application/json"}

    def upload_dashboard(
        self, json_file_path: str, dashboard_name: str, folder_uid: Optional[str] = None
    ) -> bool:
        """
        Upload a dashboard to Grafana.

        :param json_file_path: Path to the JSON file containing the dashboard definition
        :param dashboard_name: Desired name for the dashboard in Grafana
        :param folder_uid: UID of the folder to save the dashboard in (optional)
        :return: True if upload was successful, False otherwise
        """
        try:
            with open(json_file_path, "r") as file:
                dashboard_json = json.load(file)
        except Exception as e:
            print(f"Error reading JSON file: {e}")
            return False

        # Update the dashboard title
        dashboard_json["title"] = dashboard_name

        # Prepare the payload
        payload = {"dashboard": dashboard_json, "overwrite": False}

        if folder_uid:
            payload["folderUid"] = folder_uid

        # Send the request to Grafana
        response = requests.post(
            f"{self.grafana_url}/api/dashboards/db",
            headers=self.headers,
            auth=self.auth,
            data=json.dumps(payload),
        )

        if response.status_code == 200:
            print(f"Dashboard '{dashboard_name}' uploaded successfully.")
            return True
        else:
            print(
                f"Failed to upload dashboard. Status code: {response.status_code}, Response: {response.text}"
            )
            return False


# Initialize the uploader with Grafana server details and credentials
uploader = GrafanaDashboardUploader(
    grafana_url="http://localhost:8000",  # Replace with your Grafana URL and port
    username="admin",  # Replace with your Grafana username
    password="pass",  # Replace with your Grafana password
)

# Upload the dashboard
json_file_path = "./scripts/dashboard.json"
dashboard_name = "My test New Dashboard"  # Desired name for the dashboard in Grafana
folder_uid = None

uploader.upload_dashboard(json_file_path, dashboard_name, folder_uid)
