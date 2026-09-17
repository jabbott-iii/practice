from fastapi import FastAPI, HTTPException, Header, Depends
import ollama
import os
from dotenv import load_dotenv

load_dotenv()  # Load environment variables from .env file

API_KEY_CREDITS = {os.getenv('API_KEY'): 5} # Example: Each API key starts with 5 credits

app = FastAPI() # Create FastAPI app instance

def verify_api_key(x_api_key: str = Header(None)): # Dependency to verify API key and check credits
    credits = API_KEY_CREDITS.get(x_api_key, 0) # Get remaining credits for the provided API key
    if credits <= 0:
        raise HTTPException(status_code=401, detail="API Key has exhausted its credits or is invalid.")
    return x_api_key # Return the valid API key

@app.post("/generate") # Endpoint to generate text using the ministral-3-3b model
def generate_text(prompt: str, x_api_key: str = Depends(verify_api_key)): # Depend on API key verification
    API_KEY_CREDITS[x_api_key] -= 1  # Deduct one credit for each request
    response = ollama.chat(model="ministral-3:3b", messages=[{"role": "user", "content": prompt}]) # Call the Ollama API with the specified model and prompt
    return {"response": response['message']['content']} # Return the generated text in the response