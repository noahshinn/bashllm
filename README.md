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
cd ./bashllm && sudo make
```

## To Run

```
bashllm <command description>
```

## Examples
```
>>> bashllm "search for all open ports on ip"

Command:

    sudo nmap -p- <target_ip>

Explanation:

The -p- option tells nmap to scan all ports, from 1 to 65535. This is useful if you want to scan for all open ports on a target. Replace <target_ip> with the IP address or hostname of the target you want to scan.

Command has been copied to clipboard!
```

```
>>> bashllm "copy file to aws s3 bucket"

Command:

    aws s3 cp <local_file_path> s3://<bucket_name>/<destination_path>

Explanation:

Let's break down the options used in this command:

<local_file_path>: Specifies the path of the file you want to copy to the S3 bucket.
s3://<bucket_name>/<destination_path>: Specifies the destination path in the S3 bucket where you want to copy the file. Replace <bucket_name> with the name of your S3 bucket and <destination_path>

Command has been copied to clipboard!

```

```
>>> bashllm "i use zsh, read from ~/.zsh_history give me the 10 most used commands"

Command:

    history | awk '{a[$2]++}END{for(i in a){print a[i] " " i}}' | sort -rn | head -n 10

Explanation:

This command uses the history command to display the command history of the current session. The output is then piped to awk, which counts the frequency of each command and sorts them in descending order. Finally, the head command is used to display only the top 10 most used commands.

Command has been copied to clipboard!

```
