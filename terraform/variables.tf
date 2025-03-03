variable "api_token_id" {
  description = "Proxmox api_token_id"
  type        = string
  sensitive   = true
}

variable "api_token" {
  description = "Proxmox api_token"
  type        = string
  sensitive   = true
}

variable "init_password" {
  description = "Password for the pe-utils LXC container"
  type        = string
  sensitive   = true
}

variable "public_key" {
  description = "SSH Public Key"
  type        = string
  sensitive   = true
}

variable "pm_api_url" {
  description = "Proxmox API URL"
  type        = string
  sensitive   = true
}
