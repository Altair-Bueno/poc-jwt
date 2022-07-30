from typing import List
from fastapi import Depends, Header
from pydantic import BaseModel
from jose.jwt import decode

from slowapi.settings import Settings
from ..beans import settings, public_key


class AuthenticationError(Exception):
    pass


class Claims(BaseModel):
    sub: str
    exp: int
    iat: int
    iss: str
    roles: List[str]


class Authentication:
    def __init__(
        self,
        authorization: str = Header(),
        settings: Settings = Depends(settings),
        public_key: str = Depends(public_key),
    ) -> None:
        bearer = "Bearer "
        if not authorization.startswith(bearer):
            raise AuthenticationError("Invalid format")
        token = authorization.removeprefix(bearer)
        try:
            payload = decode(token, public_key, algorithms=[settings.jwt.algorithm])
            self.claims = Claims(**payload)
        except Exception as e:
            raise AuthenticationError(e)
