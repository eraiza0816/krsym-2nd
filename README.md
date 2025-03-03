# krsym-2nd

```bash
ansible-playbook playbooks/lxc_timezone_set.yml -i ../krsym-2nd/inventries/inventory.yml -l lxc --check
```

```bash
ansible-playbook -i INVENTRY.yml PLAYBOOK.yml --limit HOST_NAME --check
```

## Install
```bash
apt install ansible ansible-lint
ansible --version
ansible-lint --version
```

非推奨なインストール方法  
Docker向け．
```bash
pip install -r requirements.txt
```

### How to Encrypt & Decrypt
#### Encrypt
```bash
ansible-vault encrypt terraform/terraform.tfvars --output=terraform/terraform.tfvars.enc && rm terraform/terraform.tfvars
```

#### Decrypt (&more)
```bash
ansible-vault decrypt terraform/terraform.tfvars.enc --output=terraform/terraform.tfvars
ansible-vault view terraform/terraform.tfvars.enc
ansible-vault edit terraform/terraform.tfvars.enc
```

##### yondeoke
[ベストプラクティス](https://docs.ansible.com/ansible/2.9_ja/user_guide/playbooks_best_practices.html#playbooks-best-practices)

## Terraform
```bash
terraform init
terraform plan
terraform apply
terraform refresh
terraform state rm proxmox_lxc.cowrie
terraform plan -target=proxmox_lxc.cowrie
terraform apply -target=proxmox_lxc.cowrie
```
