from typing import List
from uuid import UUID
from fastapi import APIRouter, Depends

from slowapi.models import CreateTodo, Todo, UpdateTodo
from ..services import TodoService

router = APIRouter()


@router.get("/{uuid}")
async def get_todo(uuid: UUID, todoservice: TodoService = Depends(TodoService)) -> Todo:
    return await todoservice.get_todo(uuid)


@router.post("/{uuid}")
async def update_todo(
    uuid: UUID, update_todo: UpdateTodo, todoservice: TodoService = Depends(TodoService)
) -> Todo:
    return await todoservice.update_todo(uuid, update_todo)


@router.delete("/{uuid}")
async def remove_todo(uuid: UUID, todoservice: TodoService = Depends(TodoService)):
    await todoservice.remove_todo(uuid)


@router.get("/")
async def get_todos(todoservice: TodoService = Depends(TodoService)) -> List[Todo]:
    return await todoservice.get_todos()


@router.post("/")
async def create_todo(
    payload: CreateTodo, todoservice: TodoService = Depends(TodoService)
):
    await todoservice.create_todo(payload)
