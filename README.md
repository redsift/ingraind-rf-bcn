# ingraind-rf-bcn

RustFest '19 Barcelona InGRAINd workshop materials

We have a number of options that will get you started quickly.

 1. A Raspberry Pi
 1. Vagrant on VirtualBox
 1. A real Linux box

A `Vagrantfile` is in this repository that will bootstrap everything
you need to start working on `ingraind` and test your probes.

## Using a Raspberry Pi/Cloud server

A Raspberry Pi is provided to attendants of the workshop.  This is an
alternative to the Vagrant/VBox setup, they are provisioned with the
same script.

## Enter Vagrant

You will need 2GB free RAM, and about 8GB of free disk space to start
the VM.

The following command will bring up the box, check out the `ingraind`
repository, and build the `master` branch in debug mode.

    vagrant up

The provisioner will do a debug build to make the
subsequent builds quicker, and create a build cache. This is
important, because the first build can take up to 15 minutes or so on
a few years old laptop.

## Using a Linux box

You are going to need the following installed:

 * LLVM 9
 * Linux 4.19+
 * Clang
 * CMake
 * OpenSSL
 * CapnProto
 * Docker & Docker Compose

For a Debian Testing (Bullseye), the full list of packages can be
found in the Vagrantfile.

Note that you will need Ubuntu 9.10, or Fedora 31, both of which
provide LLVM 9.

Once you installed all the dependencies, run

    docker-compose up -d
	
Which is going to start a [NextCloud](https://nextcloud.org/)
instance on port 8080.

## Running on macOS

### Local setup

Check out this repo, and install the Rust toolchain on your computer
to get started.
You're going to need a nightly toolchain, too.

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	rustup default nightly
	
Make sure you have a working IDE configured for Rust, whether it's
VIM, Emacs, or VS Code is up to you.

### Install Vagrant/Virtualbox

If you have [brew](https://brew.sh), this should be straightforward.

    brew cask install virtualbox
    brew cask install vagrant
	
On a box that doesn't have brew, you should download
[VirtualBox](https://virtuabox.org) and
[Vagrant](https://vagrantup.com) manually, and install them from the
`.dmg` files.

You are going to need to got to _System Preferences_, authenticate,
and enable the VirtualBox system extension to continue.  This is a
mandatory step due to macOS security requirements.

## Running on Windows

I recommend using [VirtualBox](https://virtuabox.org) and
[Vagrant](https://vagrantup.com) should you use Windows 10.

You should also install a Rust nightly toolchain on Windows from
[rustup.rs](https://rustup.rs), and configure your IDE to use it.

WSL is untested, but it *may* work. You need kernel headers to
compile `ingraind`, and I have not tried accessing Microsoft's
tree. If you can get this working, please send a PR to this doc
detailing how it works.

If you happen to use [chocolatey](https://chocolatey.org/), you can do
the following steps:

    choco install vagrant
	choco install virtualbox
