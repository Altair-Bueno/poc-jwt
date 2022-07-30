from tortoise.models import Model
from tortoise.contrib.pydantic.creator import pydantic_model_creator
from tortoise.fields import *


class TodoEntity(Model):
    id = UUIDField(pk=True)
    title = CharField(max_length=20)
    content = CharField(max_length=50)


Todo = pydantic_model_creator(TodoEntity)
