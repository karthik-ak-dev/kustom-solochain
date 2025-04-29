# Manual Provisioning Steps

This document outlines the steps to manually provision a VM using Ansible.

## Prerequisites

- Access to the VM via SSH
- Terraform infrastructure already deployed
- Working IAP tunnel via `ssh.sh`

## Step-by-Step Process

### 1. Copy Ansible Files to VM

```bash
# Navigate to the infra directory
cd ./infra  # From project root
# OR
cd ../  # If you're in the ansible directory

# Use the rsync script to copy Ansible files to VM
make make_rsync
```

This copies the Ansible directory to `~/ansible` on the VM.

### 2. SSH into the VM

```bash
# Navigate to scripts directory
cd ./infra/scripts  # From project root
# OR
cd ../scripts  # If you're in the ansible directory

# SSH into the VM using the IAP tunnel
./ssh.sh
```

### 3. Install Ansible on the VM

```bash
# Navigate to the Ansible scripts directory on the VM
cd ~/ansible/scripts

# Make the installation script executable
chmod +x 00-install-ansible.sh

# Run the installation script
./00-install-ansible.sh
```

This installs Ansible and all required dependencies.

### 4. Install Ansible Role Dependencies

```bash
# Make the dependencies script executable
chmod +x 01-dependencies.sh

# Run the dependencies script
./01-dependencies.sh
```

This installs required Ansible roles like geerlingguy.docker.

### 5. Modify the Playbook for Local Execution

Edit the `~/ansible/main.yml` file on the VM to target localhost:

```yaml
---
- name: Deploy xerberus stack - whole stack per node
  # hosts: xerberus
  hosts: localhost
  connection: local
  become: true
  
  # Rest of the playbook...
```

### 6. Run the Ansible Playbook

```bash
# Navigate to the Ansible directory
cd ~/ansible

# Run the playbook
ansible-playbook main.yml
```

### 7. Verify Installation

```bash
# Check Docker installation
docker --version
docker-compose --version

# Check for Docker containers (if deploying node client)
docker ps
```

## Troubleshooting
- If you encounter permission issues, make sure you're using sudo where needed
- For module errors, check that dependencies were installed correctly
- For connection errors, verify your SSH connection is working
