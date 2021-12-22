FROM rust:latest
RUN apt-get update
RUN apt-get install -y vim && apt-get install -y clang
CMD ["/bin/bash"]
