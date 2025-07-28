FROM ghcr.io/prefix-dev/pixi:0.49.0 AS build

ARG PIXI_ENV=default

COPY . /app
WORKDIR /app

RUN pixi install --frozen -e ${PIXI_ENV}

RUN pixi run -e ${PIXI_ENV} playwright install

RUN pixi shell-hook -e ${PIXI_ENV} -s bash > /shell-hook
RUN echo "#!/bin/bash" > /app/entrypoint.sh
RUN cat /shell-hook >> /app/entrypoint.sh
RUN echo 'exec "$@"' >> /app/entrypoint.sh

ENTRYPOINT [ "/app/entrypoint.sh" ]
