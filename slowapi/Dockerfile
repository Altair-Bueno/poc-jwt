FROM python:3.10.5-bullseye
RUN mkdir /code
RUN pip install "poetry==1.1.14"
WORKDIR /code
COPY poetry.lock pyproject.toml /code/
RUN poetry install --no-interaction --no-ansi
COPY . /code
ENTRYPOINT poetry run uvicorn slowapi:app --port=9300 "--host=0.0.0.0"
