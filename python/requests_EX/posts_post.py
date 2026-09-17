import requests

url = 'https://jsonplaceholder.typicode.com/posts'

payload = {
    'title': 'Welcome to Python APIs',
    'body': 'This is a sample post created using the requests library.',
    'userId': 1
}

response = requests.post(url, json=payload) # Send POST request with JSON payload
print("Status Code:", response.status_code)
data = response.json() # Parse JSON response
print("Response Data:", data)