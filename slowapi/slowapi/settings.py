from pydantic import BaseSettings, BaseModel


class DatabaseSettings(BaseModel):
    url: str


class Settings(BaseSettings):
    database: DatabaseSettings

    class Config:
        # env_prefix = 'SLOW_'
        env_file = ".env"
        env_nested_delimiter = "_"
        case_sensitive = False
