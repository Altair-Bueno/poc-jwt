from pydantic import BaseSettings, BaseModel


class DatabaseSettings(BaseModel):
    url: str

    class Config:
        frozen = True


class JwtSettings(BaseModel):
    algorithm: str = "RSA256"
    publickey: str

    class Config:
        frozen = True


class Settings(BaseSettings):
    database: DatabaseSettings
    jwt: JwtSettings

    class Config:
        # env_prefix = 'SLOW_'
        env_file = ".env"
        env_nested_delimiter = "_"
        case_sensitive = False
        frozen = True
