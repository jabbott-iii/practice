from fastapi import FastAPI
import data

app = FastAPI()

@app.get('/dataframe')
def get_dataframe():
    return data.hourly_dataframe.to_dict(orient='records')

# other @ operations can be added here