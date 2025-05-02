FROM mirror.gcr.io/python:3.12.3-alpine
RUN apk add git openssl-dev build-base libffi-dev
ADD requirements.txt requirements.txt
RUN pip install -r requirements.txt
