FROM ubuntu:latest
WORKDIR /root
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get clean
RUN apt-get update
RUN apt-get upgrade -y

# install misc prerequisites
RUN apt-get install -y git
RUN apt-get install -y curl
RUN apt-get install -y gnupg
RUN apt-get install -y python
RUN apt-get install -y python3-pip
RUN apt-get install -y nodejs

# install yarn
RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add -
RUN echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list
RUN apt-get update
RUN apt-get install -y yarn

# install flit, turnt
RUN pip3 install flit turnt
RUN git clone https://github.com/sampsyo/bril

# install brili (interpreter)
WORKDIR /root/bril/bril-ts
RUN yarn
RUN yarn build
RUN yarn link

# install bril2json (txt --> json), bril2txt (json --> txt)
WORKDIR /root/bril/bril-txt
ENV FLIT_ROOT_INSTALL=1
RUN flit install --symlink
ENV PATH=$PATH:/root/.local/bin/

WORKDIR /root

# # install vim
# RUN apt-get install -y vim
# 
# # enable vi mode in bash
# RUN echo 'set -o vi' >> .bashrc
# 
# # install Rust
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# ENV PATH=$PATH:/root/.cargo/bin/

RUN apt-get autoremove -y
ENTRYPOINT ["/bin/bash"]
