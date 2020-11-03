# mcw-rs
A wrapper that consumes the i/o of any minecraft server and allows control of the server via rust.

# TODO
- [ ] Figure out a way to provide data to callbacks if needed?
- [ ] Keep player count
- [ ] shutdown server (but keep listening for people trying to join) if no players are online
- [ ] start server when it is detected that someone is trying to join
- [ ] let the user decide the location / file name of the server