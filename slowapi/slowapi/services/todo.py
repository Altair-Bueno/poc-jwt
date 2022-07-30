from typing import List
from uuid import UUID
from ..models import TodoEntity, Todo, CreateTodo, UpdateTodo


class TodoService:
    def __init__(self) -> None:
        pass

    async def get_todos(self) -> List[Todo]:
        entities = TodoEntity.all()
        return await Todo.from_queryset(entities)

    async def get_todo(self, id: UUID) -> Todo:
        entity = await TodoEntity.filter(id=id).first()
        return await Todo.from_tortoise_orm(entity)

    async def create_todo(self, create_todo: CreateTodo) -> Todo:
        entity = await TodoEntity.create(**create_todo.dict())
        return await Todo.from_tortoise_orm(entity)

    async def remove_todo(self, id: UUID):
        count = await TodoEntity.filter(id=id).delete()

    async def update_todo(self, id: UUID, update_todo: UpdateTodo):
        update = {k: v for k, v in update_todo.dict().items() if v}
        count = await TodoEntity.filter(id=id).update(**update)
