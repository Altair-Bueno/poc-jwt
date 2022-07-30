from typing import Optional
from pydantic.dataclasses import dataclass


@dataclass
class CreateTodo:
    title: str
    content: str


@dataclass
class UpdateTodo:
    title: Optional[str]
    content: Optional[str]
