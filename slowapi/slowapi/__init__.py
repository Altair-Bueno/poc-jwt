__version__ = "0.1.0"
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from tortoise.contrib.fastapi import register_tortoise
from .routes import router
from .settings import Settings

settings = Settings()
app = FastAPI()

app.add_middleware(
    CORSMiddleware,
    allow_credentials=True,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)
app.include_router(router)
register_tortoise(
    app,
    modules={"database": ["slowapi.models"]},
    generate_schemas=True,
    db_url=settings.database.url,
)
