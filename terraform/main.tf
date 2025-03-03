terraform {
  required_providers {
    proxmox = {
      source = "Telmate/proxmox"
      version = "2.9.14"

    }
  }
}

provider "proxmox" {
  pm_api_url = var.pm_api_url
  pm_tls_insecure = true
  pm_api_token_id = var.api_token_id
  pm_api_token_secret = var.api_token
}

resource "proxmox_lxc" "growi" {
  target_node    = "krs-n950"
  hostname       = "growi"
  ostemplate     = "ubuntu-24.04-standard_24.04-2_amd64.tar.zst"
  password       = var.init_password
  unprivileged   = true
  vmid           = 101
  memory         = 2048
  onboot         = true
  cores          = 1
  ssh_public_keys = var.public_key

  rootfs {
    storage = "local-lvm"
    size    = "45G"
  }

  network {
    name   = "eth0"
    bridge = "vmbr0"
    ip     = "192.168.200.122/24"
    gw     = "192.168.200.1"
  }

  features {
    nesting = true
  }
  nameserver  = "8.8.8.8"
}

resource "proxmox_lxc" "dns" {
  target_node = "krs-n950"
  hostname    = "dns"
  ostemplate   = "sandisk500:vztmpl/ubuntu-24.04-standard_24.04-2_amd64.tar.zst"
  cores       = 2
  memory      = 512
  onboot      = true
  vmid = 102
  rootfs {
    storage = "local-lvm"
    size    = "6G"
  }

  network {
    name   = "eth0"
    bridge = "vmbr0"
    ip     = "192.168.200.112/24"
    gw     = "192.168.200.1"
    hwaddr = "BC:24:11:BA:8A:65"
  }

  features {
    nesting = true
  }
  nameserver = "8.8.8.8"
}

resource "proxmox_lxc" "es" {
  target_node = "krs-n950"
  hostname    = "es"
  ostemplate   = "sandisk500:vztmpl/ubuntu-24.04-standard_24.04-2_amd64.tar.zst"
  cores       = 4
  memory      = 2048
  onboot      = true
  vmid = 104
  rootfs {
    storage = "local-lvm"
    size    = "5G"
  }

  network {
    name   = "eth0"
    bridge = "vmbr0"
    ip     = "192.168.200.104/24"
    gw     = "192.168.200.1"
    hwaddr = "BC:24:11:F5:40:E3"
  }

  nameserver = "192.168.200.1 192.168.200.113"
}

resource "proxmox_lxc" "collector" {
  target_node  = "krs-n950"
  hostname     = "collector"
  ostemplate   = "sandisk500:vztmpl/ubuntu-24.04-standard_24.04-2_amd64.tar.zst"
  password     = var.init_password
  unprivileged = true
  vmid = 125
  memory = 2048
  onboot = true
  cores = 1
  ssh_public_keys = var.public_key
  rootfs {
    storage = "local-lvm"
    size    = "8G"
  }
  network {
    name   = "eth0"
    bridge = "vmbr0"
    ip     = "192.168.200.125/24"
    gw     = "192.168.200.1"
    hwaddr = "BC:24:11:3A:F4:BA"
  }

  nameserver = "192.168.200.1 192.168.200.113"
}

resource "proxmox_lxc" "cowrie" {
  target_node  = "krs-n950"
  hostname     = "cowrie"
  ostemplate   = "sandisk500:vztmpl/ubuntu-24.04-standard_24.04-2_amd64.tar.zst"
  password     = var.init_password
  unprivileged = true
  vmid = 126
  memory = 2048
  onboot = true
  cores = 1
  ssh_public_keys = var.public_key
  rootfs {
    storage = "local-lvm"
    size    = "4G"
  }
  network {
    name   = "eth0"
    bridge = "vmbr0"
    ip     = "192.168.200.127/24"
    gw     = "192.168.200.1"
  }

  nameserver = "192.168.200.1 192.168.200.113"
}

resource "proxmox_lxc" "gh-runner" {
  target_node  = "krs-n950"
  hostname     = "gh-runner"
  ostemplate   = "sandisk500:vztmpl/ubuntu-24.04-standard_24.04-2_amd64.tar.zst"
  password     = var.init_password
  unprivileged = true
  vmid = 128
  memory = 1024
  onboot = true
  cores = 2
  ssh_public_keys = var.public_key
  rootfs {
    storage = "local-lvm"
    size    = "4G"
  }
  network {
    name   = "eth0"
    bridge = "vmbr0"
    ip     = "192.168.200.128/24"
    gw     = "192.168.200.1"
  }

  nameserver = "192.168.200.1 192.168.200.113"
}
