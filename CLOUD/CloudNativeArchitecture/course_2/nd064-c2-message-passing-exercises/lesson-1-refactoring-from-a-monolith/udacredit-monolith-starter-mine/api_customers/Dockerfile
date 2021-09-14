FROM python:3.7-slim

WORKDIR /backend-customers
COPY requirements.txt /backend-customers/requirements.txt
RUN pip install --upgrade pip && \
    pip install -r requirements.txt
COPY . .
