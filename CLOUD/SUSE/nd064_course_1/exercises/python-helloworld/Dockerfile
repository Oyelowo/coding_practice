FROM python:3.8
LABEL maintainer="oyelowo Oyedayo"

COPY . /app
WORKDIR /app
RUN pip install -r requirements.txt

# comand to run on container start
CMD [ "python", "app.py"]
