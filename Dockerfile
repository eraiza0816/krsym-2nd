FROM python:3.11.9-alpine
RUN apk add git openssl-dev build-base libffi-dev
ADD requirements.txt requirements.txt
RUN pip install -r requirements.txt
