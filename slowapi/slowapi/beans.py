from fastapi import Depends
from .settings import Settings
from functools import lru_cache


@lru_cache()
def settings():
    return Settings()


@lru_cache()
def public_key(settings: Settings = Depends(settings)):
    with open(settings.jwt.publickey, "rt") as file:
        return file.read()
