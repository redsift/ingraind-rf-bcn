# -*- mode: ruby -*-
# vi: set ft=ruby :

# All Vagrant configuration is done below. The "2" in Vagrant.configure
# configures the configuration version (we support older styles for
# backwards compatibility). Please don't change it unless you know what
# you're doing.
Vagrant.configure("2") do |config|
  config.vm.box = "debian/testing64"

  # Disable automatic box update checking. If you disable this, then
  # boxes will only be checked for updates when the user runs
  # `vagrant box outdated`. This is not recommended.
  # config.vm.box_check_update = false

  config.vm.network "forwarded_port", guest: 80, host: 8080
  config.vm.provider "virtualbox" do |vb|
    vb.gui = false
    vb.memory = "2048"
    vb.cpus = 2
  end

  config.vm.provision "file", source: "./docker-compose.yml", destination: "/home/vagrant/docker-compose.yml"
  config.vm.provision "shell", inline: <<-SHELL
    apt-get update
    apt-get install -y ca-certificates{,-java} \
       llvm llvm-9 libllvm9 llvm-9-dev clang libclang-9-dev clang-format-9 \
       bison flex debhelper cmake devscripts \
       zlib1g-dev libfl-dev libelf-dev libedit-dev libssl-dev \
       curl git \
       musl musl-tools musl-dev \
       linux-headers-amd64 \
       capnproto \
       pkg-config \
       docker.io docker-compose
    
    curl --proto '=https' --tlsv1.2 -sSf -o rustup.sh https://sh.rustup.rs
    sh rustup.sh -y \
        --default-toolchain nightly \
        --no-modify-path 

    echo 'source /root/.cargo/env' >> ~/.bashrc
    echo 'export KERNEL_SOURCE=/usr/src/linux-headers-5.2.0-3-amd64/' >> ~/.bashrc
    . ~/.bashrc

    rustup target add x86_64-unknown-linux-musl 
    rustup toolchain add stable 
    rustup component add rustfmt

    cargo install cargo-bpf

    systemctl enable docker
    systemctl start docker
    
    cd /home/vagrant
    docker-compose up -d

    git clone --branch v1.0 https://github.com/redsift/ingraind

    # TODO: This needs to be fixed properly
    cp -r /usr/src/linux-headers-5.2.0-3-common/* /usr/src/linux-headers-5.2.0-3-amd64/

    cd ingraind
    cargo build --release
  SHELL
end
