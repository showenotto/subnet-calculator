## Subnet Calculator
This calculator divides IP networks into smaller, manageable sub-networks (subnets). It helps network administrators quickly determine network details such as network address, broadcast address, network boundaries, usable host range, subnet masks, prefix length, etc. Using this tool will reduce human error and improve efficiency when designing or troubleshooting IP networks.

### IPv4 subnetting
IPv4 subnetting is the process of dividing a 32-bit IPv4 address space into smaller networks by extending the network prefix using a subnet mask.

Example: Inspect given subnet
<img width="1600" height="900" alt="image" src="https://github.com/user-attachments/assets/69672331-cb52-4dfa-b3a6-bab946e94d7d" />

Example: Subnet by number of hosts
<img width="1552" height="758" alt="image" src="https://github.com/user-attachments/assets/9ec5304e-0741-4929-9db8-f733c91266f7" />


#### Features
- Subnet Modes: Inspect given subnet, by number of hosts/subnets
- Copy and Export results to CSV

### IPv6 subnetting
IPv6 subnetting organizes the vastly larger 128-bit IPv6 address space into hierarchical networks using prefix lengths.

Example: Subnet by number of subnets
<img width="1562" height="756" alt="image" src="https://github.com/user-attachments/assets/f9610c8d-0386-430c-86e9-4849e684a2ef" />

Example: Subnet by hierarchy
<img width="1542" height="762" alt="image" src="https://github.com/user-attachments/assets/3d5d3057-6ac2-44e5-b730-51850b619f52" />

#### Features
- Subnet Modes: Inspect given subnet, by number of subnets and by hierarchy
- Copy and Export results to CSV (And JSON, for hierarchy mode)

### AI Assistant
NB: To use this feature you need to install ollama (https://ollama.com/) on your system and then install any AI model that is available. About 5-10 GB+ storage needed, plus decent CPU power for the AI model to run and process prompts.

It is just a chat bot that can be used to answer simple network relating questions nothing special.

<img width="1560" height="791" alt="image" src="https://github.com/user-attachments/assets/cc0b3749-9fb3-4953-a724-d0dbdc5f10e7" />


### Building the project

Linux:
```bash
dx bundle --linux --release --package-types deb
```

Windows:
```bash
dx bundle --windows --release --package-types msi
```
