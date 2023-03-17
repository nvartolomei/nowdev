# nowdev

**nowdev** is a (tiny) command line utility to manage remote development
environments on Linode. It doesn't do anything fancy and instead provides simple
commands to start/stop VMs to which persistent volumes are attached. You pay the
cost of persistent volume and VM time only when you use it (between `start` and
`stop` command).

**Why this was created?**

Scratching my own itch. I often need Linux machines a bit more powerful than the
VMs I can run on my laptop for compiling large codebases (usually ClickHouse).
There are GitHub Codespaces but they aren't as powerful/flexible as I need them
to be and Linode ends up being cheaper for me.

Works great together with
[iTerm2 tmux Integration](https://iterm2.com/documentation-tmux-integration.html),
[Visual Studio Code Remote - SSH extension](https://code.visualstudio.com/docs/remote/ssh),
[JetBrains Remote Development](https://www.jetbrains.com/remote-development/).

**TODOs:**

- Allow region to be configurable.
- Allow VM name/label to be configurable.
- Support managing multiple Linodes/VMs.

## User guide

### Using `nowdev` cli

<sub>If you haven't used this before, please follow the
[One-time setup steps](#one-time-setup-steps) first.</sub>

```
$ nowdev
A command line utility to manage remote development environments on Linode

Usage: nowdev <COMMAND>

Commands:
  start  Start a development environment
  stop   Stop a development environment
  stats  Show statistics (uptime, cost) for the development environment
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

Start a new development environment in less than 2 seconds (this starts a new
Linode/VM using the persistent volume):

```
 $ time nowdev start
Added config
https://cloud.linode.com/linodes/44054314
IP: 176.58.100.224
Writing ssh configuration to ~/.ssh/config_nowdev

real	0m1.771s
```

After you are done, stop it (this deletes the Linode/VM, but the data was
persisted on the Volume which isn't deleted):

```
 $ nowdev stop
Uptime: 0 hours and 0 minutes
Instance type: g6-standard-4
Cost: $0.06 (per hour: $0.06)
Instance deleted
```

### One-time setup steps:

<sub>Most of these steps can be automated, but it wasn't done yet.</sub>

1. First, you need a [Linode account](https://www.linode.com).

1. Create a volume named `nowdev-dev` at https://cloud.linode.com/volumes, but
   don't attach it to any Linode/VM just yet.

1. We need to manually install the operating system on the just created volume.
   Then we'll use it as the main and only volume attached to the VMs.

   1. Go to https://cloud.linode.com/linodes and the smallest possible Linode
      (i.e. Debian 11, Nanode 1 GB with 25 GB storage, add your SSH key). We'll
      use it to clone the operating system to the persistent volume created in
      the previous step. Note that the persistent volume must be sized at least
      to the storage of this Linode so that volume cloning/imaging is possible.

   1. Reboot the Linode into the
      [rescue mode](https://www.linode.com/docs/guides/rescue-and-rebuild/#booting-into-rescue-mode)
      straight away and choose to mount the `nowdev-dev` persistent volume to
      `/dev/sdc`.

   1. [Launch LISH console](https://www.linode.com/docs/guides/rescue-and-rebuild/#connecting-to-a-linode-running-in-rescue-mode)
      from the UI.

      1. Now we need to clone/image the VM disk `/dev/sda` to the persistent
         volume `/dev/sdc`:

      1. Verify that we have all the block devices present:

         ```
         root@finnix:~# lsblk
         NAME  MAJ:MIN RM   SIZE RO TYPE MOUNTPOINT
         loop0   7:0    0 425.2M  1 loop /usr/lib/live/mount/rootfs/filesystem.squashfs
         sda     8:0    0  24.5G  0 disk
         sdb     8:16   0   512M  0 disk
         sdc     8:32   0    30G  0 disk
         sr0    11:0    1   503M  0 rom  /run/live/medium
         zram0 252:0    0 492.6M  0 disk [SWAP]
         ```

      1. Start cloning and wait for it to finish:

         ```
         root@finnix:~# pv < /dev/sda > /dev/sdc
         2.94GiB 0:00:09 [ 343MiB/s] [===>                              ] 12% ETA 0:01:05
         ```

   1. The VM isn't needed past this point. You can delete it now.

1. Your persistent volume with an operating system is now ready to be used for
   the development environment when you need it. At this moment you are only
   charged for the volume ($1 per 10 GB in 2023).

1. The `nowdev` cli needs to interact with the Linode API. You need to
   [create a personal access token](https://www.linode.com/docs/products/tools/api/guides/manage-api-tokens/)
   and store it at `~/.config/nowdev/token`.

   ```
   mkdir -p ~/.config/nowdev
   touch ~/.config/nowdev/token
   chmod 0600 ~/.config/nowdev/token
   echo -n "mytoken" > ~/.config/nowdev/token
   ```

1. Now you can use the `nowdev` cli to start the development environment. It
   will also write ssh configs to `~/.ssh/config_nowdev`. Add to your
   `~/.ssh/config` this line `Include config_nowdev` so that you could just run
   `ssh nowdev-dev` to connect to it.

### Troubleshooting

- If networking doesn't work or you can't connect via ssh, you might need to
  re-configure networking with DHCP as the VM will get new IPs every time it
  boots. E.g. this is what I use for Debian:

  ```
  $ cat /etc/network/interfaces
  auto lo
  iface lo inet loopback
  source /etc/network/interfaces.d/*
  auto eth0
  allow-hotplug eth0
  iface eth0 inet dhcp
  ```

- If it takes a long time for the (Debian) instance to boot, likely it is
  because `systemd` is waiting for the swap device to appear. Just disable it by
  running the following command as root:

  ```
  sed -i '/swap/d' /etc/fstab
  ```

## Development

```sh
brew install openapi-generator
```

```sh
pushd linode/schema-transformer
npm ci
node index.mjs
popd
```

```sh
openapi-generator generate --generator-name rust \
  --package-name linode-api \
  --input-spec ./linode/transformed-schema.json \
  --global-property apis,apiDocs=false,models,modelDocs=false,supportingFiles \
  --output linode-api

cargo fmt --package linode-api && cargo clippy --fix --package linode-api --allow-staged --allow-dirty
```
