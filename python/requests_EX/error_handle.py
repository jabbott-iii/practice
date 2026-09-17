import requests

try:
    response = requests.get(url='https://jsonplaceholder.typicode.com/invalid_endpoint', timeout=5)  # Intentionally incorrect URL to trigger an error
    response.raise_for_status()  # Raise an error for bad responses (4xx and 5xx)
    print('Success', response.json())
except requests.exceptions.Timeout: # Handle timeout exception
    print('The request timed out')
except requests.exceptions.RequestException as e: # Catch all other request-related errors
    print('An error occurred:', e)