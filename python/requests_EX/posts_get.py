import requests

url = 'https://jsonplaceholder.typicode.com/posts/1'

response = requests.get(url) # Send GET request

print("Status Code:", response.status_code)
print("Content Type:", response.headers.get('Content-Type'))

data = response.json() # Parse JSON response

print("Title:", data['title']) # Print the title of the post
