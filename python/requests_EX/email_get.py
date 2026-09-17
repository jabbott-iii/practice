import requests

url = 'https://jsonplaceholder.typicode.com/users/'
params = {'page': 2} # Example query parameter

response = requests.get(url, params=params) # Send GET request with parameters
print('Final URL:', response.url)
print("Status Code:", response.status_code)

data = response.json() # Parse JSON response

for user in data: # Iterate through user data
    print(f"ID: {user['id']}, Name: {user['name']}, Email: {user['email']}")