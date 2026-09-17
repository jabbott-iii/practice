import requests

token = 'your_api_token_here'  # Replace with your actual token
base_url = 'https://api.example.com/data' # Replace with the actual API endpoint

headers = {
    'Authorization': f'Bearer {token}'
}

response = requests.get(base_url, headers=headers) # Send GET request with authorization header
print("Status Code:", response.status_code)

try:    
    data = response.json() # Parse JSON response
    print("Response Data:", data)
except ValueError:
    print("Response content is not valid JSON")
    print("Response Text:", response.text)