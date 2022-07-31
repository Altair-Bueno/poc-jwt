from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from tortoise.contrib.fastapi import register_tortoise
from .routes import router
from .beans import settings
from .models.database import MODULES

settings = settings()

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
    modules=MODULES,
    generate_schemas=True,
    db_url=settings.database.url,
)
