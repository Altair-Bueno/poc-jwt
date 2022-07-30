from typing import Optional
from tortoise.models import Model
from tortoise.contrib.pydantic.creator import pydantic_model_creator
from tortoise.fields import *
from pydantic import BaseModel


class CreateTodo(BaseModel):
    title: str
    content: str


class UpdateTodo(BaseModel):
    title: Optional[str]
    content: Optional[str]


class TodoEntity(Model):
    id = UUIDField(pk=True)
    title = CharField(max_length=20)
    content = CharField(max_length=50)


Todo = pydantic_model_creator(TodoEntity)
