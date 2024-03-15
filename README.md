# 名はない - Intruder
Malware made for educational purposes. This was mainly made as a submission for a hackathon, please read the disclaimer.

# Objective & Limits
- Objectives
- - Make a harmless malware.
- - Can not harm the computer (software/hardware).
- - Aims to be anoying.
- Limits
- - No Elevated Priviledges.
- - Do not allow full access from the intruder side.

# How does it work?
There are 2 sections to this part of the program.
# Intruder
* UI: Shows a nice interface & communicates with the worker thread to perform any actions.
* Worker Thread
* * TCP Connection: Listens for TCP connections
* * Communication: Listens for messages from the main thread
# Victim
* Connects to a TCP server
* Await for TCP messages from the server and do different actions based on the received packets.

# Communications
By default the [`BUFFER_SIZE`](./nawanai-victim/src/main.rs#L43) is 6. Meaning an array of 6 bytes is in charge of communications. Both sides (intruder & victim) need have the same flags for the different actions to be executed.

# Team
Members of the 名はない team:
- [yxl-prz](https://github.com/yxl-prz) (Me)
- [PotatoAle](https://github.com/PotatoAle)

# Other Information
This project was rated
- 90/100 by Vasilhs S (Computer Science Masters)
- 90/100 by Ivan G (Computer Science Masters)
# Disclaimer
This software is provided for educational and entertainment purposes only. The author and contributors do not condone or support any misuse of this software. The user is solely responsible for any actions taken using this software. By downloading, installing, or using this software, the user agrees to take full responsibility for any consequences that may arise.
