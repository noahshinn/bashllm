# BashLLM
Faster search for shell commands.

## Requirements
`rust`

## Installation

Clone this repo:
```
git clone https://github.com/noahshinn024/bashllm
```

Run `make`:
```
cd ./bashllm && make
```

## To Run

```
bashllm <command description>
```

## Examples
```
>>> bashllm "search for all open ports on ip"
sudo nmap <ip addr>
```

```
>>> bashllm "i use zsh, read from ~/.zsh_history give me the 10 most used commands"
cat ~/.zsh_history | sort | uniq -c | sort -nr | head -10
```
