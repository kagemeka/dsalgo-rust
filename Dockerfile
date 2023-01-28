FROM rust:latest 

COPY install_deps.sh .
RUN ./install_deps.sh && rm install_deps.sh